use std::collections::HashSet;

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn parse(text: &str) -> Self {
        let edges = text.lines().enumerate().map(|(i, line)| {
            let (l, r) = line.split_once(" <-> ").unwrap();
            assert_eq!(l.parse::<usize>().unwrap(), i);
            r.split(", ").map(|s| s.parse::<usize>().unwrap()).collect()
        }).collect();
        Self { edges }
    }

    fn dfs(&self, start: usize) -> HashSet<usize> {
        let mut visited = HashSet::from([start]);
        let mut next = vec![start];
        while let Some(id) = next.pop() {
            for adj in &self.edges[id] {
                if !visited.contains(adj) {
                    next.push(*adj);
                    visited.insert(*adj);
                }
            }
        }
        visited
    }

    fn groups(&self) -> usize {
        let mut rest = HashSet::from_iter(0..self.edges.len());
        let mut count = 0;
        while let Some(start) = rest.iter().next().cloned() {
            let group = self.dfs(start);
            rest = &rest ^ &group;
            count += 1;
        }
        count
    }
}

pub fn run(content: &str) {
    let graph = Graph::parse(content);
    println!("{} {}", graph.dfs(0).len(), graph.groups());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        0 <-> 2\n\
        1 <-> 1\n\
        2 <-> 0, 3, 4\n\
        3 <-> 2, 4\n\
        4 <-> 2, 3, 6\n\
        5 <-> 6\n\
        6 <-> 4, 5";

    #[test]
    fn small() {
        let graph = super::Graph::parse(TEST);
        assert_eq!(graph.dfs(0).len(), 6);
    }

    #[test]
    fn large() {
        let graph = super::Graph::parse(TEST);
        assert_eq!(graph.groups(), 2);
    }
}
