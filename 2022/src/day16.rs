use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Valve {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn from(input: &str) -> Valve {
        let pattern = Regex::new("\
            Valve (\\w+) has flow rate=(\\d+); \
            tunnels? leads? to valves? (.+)"
        ).unwrap();
        let caps = pattern.captures(input).unwrap();
        let get = |i: usize| caps.get(i).unwrap().as_str();

        let name = get(1).to_string();
        let flow = get(2).parse::<u32>().unwrap();
        let tunnels = get(3).split(", ").map(|s| s.to_string()).collect();
        Valve { name, flow, tunnels }
    }
}

#[derive(Debug)]
struct Volcano {
    nodes: Vec<Valve>,
    lookup: HashMap<String, usize>,
}

impl Volcano {
    fn from(input: &str) -> Volcano {
        let nodes: Vec<Valve> = input.lines().map(Valve::from).collect();
        let lookup = HashMap::from_iter(
            nodes.iter().enumerate().map(|(k, v)| (v.name.clone(), k)));
        Volcano { nodes, lookup }
    }

    #[allow(dead_code)]
    fn flow(&self, mask: u64) -> u32 {
        (0..self.nodes.len()).map(|i|
            if mask & (1 << i) != 0 {self.nodes[i].flow} else {0}
        ).sum()
    }

    fn traverse(&self, limit: u32, init_mask: u64) -> (u64, u32) {
        type State = (usize, u64, u32, u32);  // index, mask, score, step
        let init_pos = *self.lookup.get("AA").unwrap();
        let mut queue = VecDeque::<State>::from([(init_pos, init_mask, 0, 0)]);
        let mut best = HashMap::<(usize, u64), u32>::new();

        while let Some((index, mask, score, step)) = queue.pop_front() {
            if step >= limit { break; }
            if let Some(&n) = best.get(&(index, mask)) {
                if score <= n { continue; }
            }

            let valve = &self.nodes[index];
            if mask & (1 << index) == 0 && valve.flow > 0 {
                let new_mask = mask | (1 << index);
                let new_score = score + valve.flow * (limit - step - 1);
                queue.push_back((index, new_mask, new_score, step + 1));
            }
            for dest in &valve.tunnels {
                let new_index = *self.lookup.get(dest).unwrap();
                queue.push_back((new_index, mask, score, step + 1));
            }
            best.insert((index, mask), score);
        }
        let mut result: (u64, u32) = (0, 0);
        for ((_, k), v) in best {
            if v > result.1 { result = (k ^ init_mask, v); }
        }
        result
    }

    fn traverse_one(&self, limit: u32) -> u32 {
        self.traverse(limit, 0).1
    }

    fn traverse_two(&self, limit: u32) -> u32 {
        let mut result: u32 = 0;
        let p: Vec<usize> = self.nodes.iter().enumerate().filter_map(
            |(k, v)| if v.flow > 0 {Some(k)} else {None}).collect();
        for m in 0..(1 << p.len() - 1) {
            let split_mask: u64 = (0..p.len()).map(
                |i| if m & (1 << i) != 0 {1 << p[i]} else {0}).sum();
            let (_, score1) = self.traverse(limit, split_mask);
            let (_, score2) = self.traverse(limit, !split_mask);
            let score = score1 + score2;
            if score > result { result = score; }
        }
        result
    }
}

pub fn run(content: &str) {
    let inst = Volcano::from(content);
    println!("{} {}", inst.traverse_one(30), inst.traverse_two(26));
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { "\
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
        Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
        Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
        Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
        Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
        Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
        Valve HH has flow rate=22; tunnel leads to valve GG\n\
        Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
        Valve JJ has flow rate=21; tunnel leads to valve II"
    }

    #[test]
    pub fn valves() {
        let inst = super::Volcano::from(example());
        assert_eq!(inst.traverse_one(30), 1651);
        assert_eq!(inst.traverse_two(26), 1707);
    }
}
