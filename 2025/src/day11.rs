use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Graph {
    nodes: HashMap<String, HashSet<String>>,
    visited: RefCell<HashMap<String, usize>>,
}

impl Graph {
    fn parse(text: &str) -> Self {
        let nodes = HashMap::from_iter(text.lines().map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let edges = HashSet::from_iter(r.split(' ').map(|s| s.into()));
            (l.into(), edges)
        }));
        let init = ("you".into(), 1);
        Self { nodes, visited: RefCell::new(HashMap::from([init])) }
    }

    fn count_paths(&self, node: &str) -> usize {
        if let Some(count) = self.visited.borrow().get(node) {
            return *count;
        }
        let count = self.nodes.iter().filter_map(|(k, v)| {
            if v.contains(node) {Some(self.count_paths(k))} else {None}
        }).sum::<usize>();
        self.visited.borrow_mut().insert(node.into(), count);
        count
    }

    fn count_passing(&self, points: Vec<&str>) -> usize {
        points.windows(2).rev().map(|a| {
            let mut cache = self.visited.borrow_mut();
            cache.clear();
            cache.insert(a[0].into(), 1);
            drop(cache);
            self.count_paths(a[1])
        }).fold(1, |a, b| a * b)
    }

    fn count_special(&self) -> usize {
        self.count_passing(vec!["svr", "dac", "fft", "out"]) +
        self.count_passing(vec!["svr", "fft", "dac", "out"])
    }
}

pub fn run(content: &str) {
    let graph = Graph::parse(content);
    println!("{} {}", graph.count_paths("out"), graph.count_special());
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let test = super::Graph::parse("\
            aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\n\
            ddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\n\
            iii: out");
        assert_eq!(test.count_paths("out"), 5);
    }

    #[test]
    fn large() {
        let test = super::Graph::parse("\
            svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\n\
            ccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\n\
            fff: ggg hhh\nggg: out\nhhh: out");
        assert_eq!(test.count_special(), 2);
    }
}
