
#[derive(Clone, Copy, Debug)]
enum Operation {
    AddX(i32),
    Noop,
}

#[derive(Debug)]
struct Program {
    ops: Vec<Operation>,
}

impl Program {
    fn from(input: &str) -> Program {
        let ops = input.lines().map(|s| {
            let a: Vec<&str> = s.split_whitespace().collect();
            match a[..] {
                ["addx", n] => Operation::AddX(n.parse::<i32>().unwrap()),
                ["noop"] => Operation::Noop,
                _ => panic!("Incorrect input")
            }
        }).collect();
        Program { ops }
    }

    fn iter(&self) -> ProgramIter {
        ProgramIter {
            ops: &self.ops[..],
            pos: 0,
            reg: 1,
            cur: Operation::Noop,
        }
    }
}

struct ProgramIter<'a> {
    ops: &'a [Operation],
    pos: usize,
    reg: i32,
    cur: Operation,
}

impl<'a> Iterator for ProgramIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.reg;
        if let Operation::AddX(n) = self.cur {
            self.reg += n;
            self.cur = Operation::Noop;
            return Some(value);
        }
        if self.pos == self.ops.len() {
            return None;
        }
        self.cur = self.ops[self.pos];
        self.pos += 1;
        Some(value)
    }
}

pub fn run(content: &str) {
    let inst = Program::from(content);
    let signal: i32 = inst.iter().enumerate().map(|(k, v)|
        (k as i32 + 1) * v).skip(19).step_by(40).take(6).sum();
    println!("{}", signal);

    for (k, v) in inst.iter().enumerate() {
        let x = (k % 40) as i32;
        let fill = (v-1..=v+1).contains(&x);
        print!("{}", if fill {'#'} else {'.'});
        if x == 39 { println!(); }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn program() {
        let inst = super::Program::from("noop\naddx 3\naddx -5");
        let data: Vec<i32> = inst.iter().collect();
        assert_eq!(data, [1, 1, 1, 4, 4]);
    }
}
