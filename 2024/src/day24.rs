use rand::{RngCore, SeedableRng, rngs::StdRng};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq)]
enum Operation { And, Or, Xor }

#[derive(Clone, Debug)]
struct Gate {
    op: Operation,
    left: String,
    right: String,
    output: String,
}

#[derive(Clone)]
struct Computer {
    data: HashMap<String, u8>,
    gates: Vec<Gate>,
}

impl Computer {
    fn parse(text: &str) -> Self {
        let (l, r) = text.split_once("\n\n").unwrap();
        let data = l.lines().map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            (l.to_owned(), r.parse::<u8>().unwrap())
        }).collect::<HashMap<_, _>>();
        let gates = r.lines().map(|line| {
            let (l, r) = line.split_once(" -> ").unwrap();
            let a = l.split(" ").collect::<Vec<_>>();
            Gate {
                op: match a[1] {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    "XOR" => Operation::Xor,
                    _ => panic!(),
                },
                left: a[0].to_owned(),
                right: a[2].to_owned(),
                output: r.to_owned(),
            }
        }).collect::<Vec<_>>();
        Self { data, gates }
    }

    fn process(&mut self) -> u64 {
        let mut lookup = HashMap::new();
        for (idx, gate) in self.gates.iter().enumerate() {
            for input in [&gate.left, &gate.right] {
                lookup.entry(input.to_owned())
                    .or_insert_with(|| vec![]).push(idx);
            }
        }
        let mut queue = self.data.keys().cloned().collect::<VecDeque<_>>();
        let empty = vec![];
        while let Some(key) = queue.pop_front() {
            let mut unresolved = 0;
            for idx in lookup.get(&key).unwrap_or(&empty) {
                let gate = &self.gates[*idx];
                if self.data.contains_key(&gate.output) { continue; }
                let left = self.data.get(&gate.left);
                let right = self.data.get(&gate.right);
                if left.is_some() && right.is_some() {
                    let (l, r) = (left.unwrap(), right.unwrap());
                    let (k, v) = (gate.output.clone(), match gate.op {
                        Operation::And => l & r,
                        Operation::Or => l | r,
                        Operation::Xor => l ^ r,
                    });
                    queue.push_back(k.clone());
                    self.data.insert(k, v);
                } else {
                    unresolved += 1;
                }
            }
            if unresolved > 0 {
                queue.push_back(key);
            }
        }
        (0..self.rank()).map(|i| {
            let key = format!("z{i:02}");
            (*self.data.get(&key).unwrap() as u64) << i
        }).sum()
    }

    fn find_output(&self, z: &str) -> Option<usize> {
        self.gates.iter().enumerate().filter_map(|(k, v)| {
            if v.output == z {Some(k)} else {None}
        }).next()
    }

    fn rank(&self) -> usize {
        (0..=64).skip_while(|&i| {
            let key = format!("z{:02}", i);
            self.find_output(&key).is_some()
        }).next().unwrap()
    }

    fn depends(&self, output: &str) -> Vec<String> {
        let mut result = HashSet::new();
        let mut queue = vec![output.to_owned()];
        while let Some(key) = queue.pop() {
            let gate = self.gates.iter().filter(|x| x.output == key)
                .next().unwrap();
            for input in [&gate.left, &gate.right] {
                let xy = input.starts_with('x') || input.starts_with('y');
                if result.insert(input.clone()) && !xy {
                    queue.push(input.clone());
                }
            }
        }
        let mut output = Vec::from_iter(result.into_iter());
        output.sort();
        output
    }

    fn find_index(&self, op: Operation, in1: &str, in2: Option<&str>)
    -> Option<usize> {
        let mut it = self.gates.iter().enumerate().filter_map(|(k, v)| {
            let l = v.left == in1 && in2.is_none_or(|s| v.right == s);
            let r = v.right == in1 && in2.is_none_or(|s| v.left == s);
            if (l || r) && v.op == op {Some(k)} else {None}
        });
        let (v1, v2) = (it.next(), it.next());
        assert!(v2.is_none());
        v1
    }

    fn find_carry(&self) -> Vec<(String, bool, Option<usize>)> {
        let mut result = vec![];
        for i in 1..self.rank() - 1 {
            let (x, y, z) =
                (format!("x{i:02}"), format!("y{i:02}"), format!("z{i:02}"));
            let mut swap = None;
            let p = self.find_index(Operation::Xor, &x, Some(&y)).unwrap();
            let v = &self.gates[p].output;
            let u = self.find_index(Operation::Xor, v, None).map(|p| {
                let g = &self.gates[p];
                if g.output != z { swap = Some(p); }
                if g.left == *v {&g.right} else {&g.left}
            });
            result.push((u.unwrap_or(v).to_owned(), u.is_some(), swap));
        }
        result
    }

    fn find_depends(&self, n: usize) -> Vec<String> {
        let result = self.gates.iter().filter(|gate| {
            let (mut mx, mut my) = (0_u64, 0_u64);
            for key in self.depends(&gate.output) {
                let (is_x, is_y) = (key.starts_with('x'), key.starts_with('y'));
                if !is_x && !is_y { continue; }
                let mask = 1 << &key[1..].parse::<u32>().unwrap();
                if is_x {mx |= mask} else {my |= mask};
            }
            mx == my && mx == (1 << n) - 1
        }).collect::<Vec<_>>();
        result.into_iter().map(|x| x.output.to_owned()).collect()
    }

    fn swap_pair(&mut self, p1: usize, p2: usize) -> [String; 2] {
        let t1 = self.gates[p1].output.to_owned();
        let t2 = self.gates[p2].output.to_owned();
        self.gates[p1].output = t2.clone();
        self.gates[p2].output = t1.clone();
        [t1, t2]
    }

    fn fix_carry(&mut self) -> Vec<String> {
        let mut result = vec![];
        let carry_data = self.find_carry();
        for (idx, (out, ok, pos)) in carry_data.iter().enumerate() {
            if let Some(p1) = pos {
                let key = format!("z{:02}", idx + 1);
                let p2 = self.find_output(&key).unwrap();
                result.extend_from_slice(&self.swap_pair(*p1, p2));
            }
            if !ok {
                let a = self.find_depends(idx + 1).into_iter().filter_map(|s| {
                    let p = self.find_index(Operation::Xor, &s, None)?;
                    let g = &self.gates[p];
                    Some(if g.left == s {&g.right} else {&g.left})
                }).collect::<Vec<_>>();
                let p1 = self.find_output(a[0]).unwrap();
                let p2 = self.find_output(out).unwrap();
                result.extend_from_slice(&self.swap_pair(p1, p2));
            }
        }
        result
    }
}

fn _verify(comp: Computer, bits: usize) {
    let mut rng = StdRng::seed_from_u64(0);
    let mask = (1_u64 << bits) - 1;
    let rank = comp.rank();
    loop {
        let mut c = comp.clone();
        let x = rng.next_u64() & mask;
        let y = rng.next_u64() & mask;
        for (ch, val) in [('x', x), ('y', y)] {
            for i in 0..rank {
                let key = format!("{}{:02}", ch, i);
                c.data.insert(key, ((val >> i) & 1) as u8);
            }
        }
        let res = c.process();
        assert_eq!(res, x + y);
        print!(".");
    }
}

pub fn run(content: &str) {
    let mut inst = Computer::parse(content);
    let out = inst.clone().process();
    let mut res = inst.fix_carry();
    res.sort();
    println!("{} {}", out, res.join(","));
    // _verify(inst, 45);
}

#[cfg(test)]
mod tests {
    const TEST_1: &str = "\
        x00: 1\nx01: 1\nx02: 1\n\
        y00: 0\ny01: 1\ny02: 0\n\n\
        x00 AND y00 -> z00\nx01 XOR y01 -> z01\nx02 OR y02 -> z02";
    const TEST_2: &str = "\
        x00: 1\nx01: 0\nx02: 1\nx03: 1\nx04: 0\n\
        y00: 1\ny01: 1\ny02: 1\ny03: 1\ny04: 1\n\n\
        ntg XOR fgs -> mjb\ny02 OR x01 -> tnw\nkwq OR kpj -> z05\n\
        x00 OR x03 -> fst\ntgd XOR rvg -> z01\nvdt OR tnw -> bfw\n\
        bfw AND frj -> z10\nffh OR nrd -> bqk\ny00 AND y03 -> djm\n\
        y03 OR y00 -> psh\nbqk OR frj -> z08\ntnw OR fst -> frj\n\
        gnj AND tgd -> z11\nbfw XOR mjb -> z00\nx03 OR x00 -> vdt\n\
        gnj AND wpb -> z02\nx04 AND y00 -> kjc\ndjm OR pbm -> qhw\n\
        nrd AND vdt -> hwm\nkjc AND fst -> rvg\ny04 OR y02 -> fgs\n\
        y01 AND x02 -> pbm\nntg OR kjc -> kwq\npsh XOR fgs -> tgd\n\
        qhw XOR tgd -> z09\npbm OR djm -> kpj\nx03 XOR y03 -> ffh\n\
        x00 XOR y04 -> ntg\nbfw OR bqk -> z06\nnrd XOR fgs -> wpb\n\
        frj XOR qhw -> z04\nbqk OR frj -> z07\ny03 OR x01 -> nrd\n\
        hwm AND bqk -> z03\ntgd XOR rvg -> z12\ntnw OR pbm -> gnj";

    #[test]
    fn small() {
        let process = |s| super::Computer::parse(s).process();
        assert_eq!(process(TEST_1), 4);
        assert_eq!(process(TEST_2), 2024);
    }
}
