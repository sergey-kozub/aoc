use std::collections::{HashMap, HashSet};

type Link = (u32, u32);

#[derive(Debug)]
struct Links {
    data: Vec<Link>,
    twin: Vec<u32>,
}

#[derive(Debug)]
struct Node {
    index: u32,
    edges: Vec<u32>,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    exits: Vec<Vec<u32>>,
    pipes: Vec<Vec<u32>>,
}

#[derive(Debug)]
struct GraphIter {
    edges: Vec<Link>,
    nodes: usize,
    total: u64,
}

fn ordered(a: u32, b: u32) -> (u32, u32) {
    (a.min(b), a.max(b))
}

impl Links {
    fn parse(text: &str) -> Self {
        let data = text.lines().map(|line| {
            let (l, r) = line.split_once('/').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        }).collect::<Vec<_>>();
        let twin = data.iter().filter_map(|(l, r)| {
            if *l == *r {Some(*l)} else {None}
        }).collect();
        Self { data, twin }
    }

    fn score(&self, path: &[Link]) -> (u32, usize) {
        let (mut result, mut extra) = (0, 0);
        let mut twin = HashSet::<u32>::from_iter(self.twin.iter().cloned());
        for (l, r) in path {
            result += l + r;
            if twin.take(&l).is_some() { result += l * 2; extra += 1; }
            if twin.take(&r).is_some() { result += r * 2; extra += 1; }
        }
        (result, extra)
    }

    fn find_best(&self) -> (u32, u32) {
        let graph = Graph::build(self);
        let iter = GraphIter::new(&graph);
        let mut best = 0;
        let mut longest = (0, 0);

        let start = graph.exits.iter().filter_map(|x| {
            if x[0] == 0 {Some(x[x.len() - 1])} else {None}
        }).collect::<Vec<_>>();
        let exclude = if start.len() == 2 {
            let link = ordered(start[0], start[1]);
            let p = iter.edges.iter().position(|&x| x == link).unwrap();
            Some(1_u64 << p)
        } else {None};

        for mask in 0..iter.total {
            if !iter.is_valid(mask) { continue; }
            if exclude.is_some_and(|x| (mask & x) != 0) { continue; }

            let hist = iter.histogram(mask);
            let odd = hist.iter().enumerate().filter_map(|(k, v)| {
                if v % 2 == 1 {Some(k as u32)} else {None}
            }).collect::<Vec<_>>();
            assert!(odd.is_empty() || odd.len() == 2);

            for i in 0..2 {
                // Find entry/exit options.
                let (s, t) = if odd.is_empty() {(start[i], start[i])}
                    else {(odd[i], odd[1 - i])};
                let enter = graph.exits.iter()
                    .filter(|a| a[0] == 0 && a[a.len() - 1] == s)
                    .cloned().collect::<Vec<_>>();
                if enter.is_empty() { continue; }

                let mut exit = graph.exits.iter()
                    .filter(|a| a[0] != 0 && a[a.len() - 1] == t)
                    .cloned().collect::<Vec<_>>();
                exit.push(vec![]);

                // Build traversal path.
                let mut links = enter[0].windows(2)
                    .map(|a| ordered(a[0], a[1])).collect::<Vec<_>>();
                for (k, v) in iter.edges.iter().enumerate() {
                    if (mask & (1 << k)) == 0 { continue; }
                    for link in graph.connection(v.0, v.1) {
                        links.push(link);
                    }
                }

                // Try possible exits.
                for path in exit {
                    let mut full = links.clone();
                    for a in path.windows(2) {
                        full.push(ordered(a[0], a[1]));
                    }
                    let test = HashSet::<Link>::from_iter(full.iter().cloned());
                    if test.len() != full.len() { continue; }

                    let (score, extra) = self.score(&full);
                    best = best.max(score);
                    let key = (full.len() + extra, score);
                    if key > longest { longest = key; }
                }
            }
        }
        (best, longest.1)
    }
}

impl Graph {
    fn build(links: &Links) -> Self {
        // Build map: node => linked nodes
        let mut hist: Vec<Vec<u32>> = vec![];
        for &(l, r) in links.data.iter() {
            if l == r { continue; }
            let size = l.max(r) as usize + 1;
            if hist.len() < size { hist.resize(size, vec![]); }
            hist[l as usize].push(r);
            hist[r as usize].push(l);
        }

        // Extract sequences of terminal nodes.
        let mut tails: Vec<Vec<u32>> = vec![];
        while let Some(pos) = hist.iter().position(|x| x.len() == 1) {
            let mut pos = pos as u32;
            let mut seq = vec![pos];
            while let Some(next) = hist[pos as usize].pop() {
                seq.push(next);
                let slot = &mut hist[next as usize];
                slot.retain(|&x| x != pos);
                if slot.len() > 1 || next == 0 { break; }
                pos = next;
            }
            for prev in tails.iter_mut() {
                let last = prev[prev.len() - 1];
                if last == 0 { continue; }
                if let Some(pos) = seq.iter().position(|&x| x == last) {
                    for x in &seq[pos + 1..] { prev.push(*x); }
                }
            }
            tails.push(seq);
        }

        // Extract sequences of chained links.
        let mut pipes: Vec<Vec<u32>> = vec![];
        while let Some(pos) = hist.iter().position(|x| x.len() == 2) {
            let (mut p0, mut p1) = (pos, hist[pos][0]);
            while hist[p0].len() == 2 {
                let (l, r) = (hist[p0][0], hist[p0][1]);
                (p0, p1) = (if l == p1 {r} else {l} as usize, p0 as u32);
                if p0 == pos { break; }  // cycle
            }
            let mut seq = vec![p0 as u32, p1];
            let (mut p1, mut p2) = (p0 as u32, p1 as usize);
            loop {
                let l = hist[p2].pop().unwrap();
                let r = hist[p2].pop().unwrap();
                (p1, p2) = (p2 as u32, if l == p1 {r} else {l} as usize);
                seq.push(p2 as u32);
                if hist[p2].len() != 2 { break; }
            }
            pipes.push(seq);
        }

        // Recalculate paths that reach the graph.
        let mut exits: Vec<Vec<u32>> = vec![];
        let xmap = pipes.iter().enumerate().flat_map(|(k, v)| {
            v[1..v.len() - 1].iter().map(move |&x| (x, k))
        }).collect::<HashMap<u32, usize>>();
        for tail in tails {
            let last = tail[tail.len() - 1];
            match xmap.get(&last) {
                Some(&i) => {
                    let pipe = &pipes[i as usize];
                    let j = pipe.iter().position(|&x| x == last).unwrap();
                    let l = tail.iter().cloned().chain(
                        pipe[j + 1..].iter().copied()).collect::<Vec<_>>();
                    let r = tail.into_iter().chain(
                        pipe[..j].iter().rev().copied()).collect::<Vec<_>>();
                    exits.push(l);
                    exits.push(r);
                },
                None => exits.push(tail),
            }
        }

        // Build graph with connected nodes.
        let nodes = hist.iter().enumerate().filter_map(|(k, v)| {
            if v.is_empty() { return None; }
            let k = k as u32;
            let edges = v.iter().map(|&x| {
                let p1 = pipes.iter().position(|a| a[..2] == [k, x]);
                let p2 = pipes.iter().position(|a| a[a.len() - 2..] == [x, k]);
                if p1.is_some() || p2.is_some() {
                    let last = p1.is_some();
                    let a = &pipes[p1.or(p2).unwrap()];
                    if last {a[a.len() - 1]} else {a[0]}
                } else {x}
            }).collect();
            Some(Node { index: k, edges })
        }).collect();
        Self { nodes, exits, pipes }
    }

    fn connection(&self, l: u32, r: u32) -> Vec<Link> {
        let key = ordered(l, r);
        let pos = self.pipes.iter().position(|a| {
            ordered(a[0], a[a.len() - 1]) == key
        });
        match pos {
            Some(i) => self.pipes[i].windows(2)
                .map(|a| ordered(a[0], a[1])).collect(),
            None => vec![key],
        }
    }
}

impl GraphIter {
    fn new(graph: &Graph) -> Self {
        let mut edges = vec![];
        for node in &graph.nodes {
            for i in &node.edges {
                if *i < node.index { continue; }
                edges.push((node.index, *i));
            }
        }
        let nodes = edges.iter().map(|x| x.1).max().unwrap() as usize + 1;
        let total = 1 << edges.len();
        Self { edges, nodes, total }
    }

    fn is_valid(&self, mask: u64) -> bool {
        let mut result = 0_u64;
        for i in 0..self.edges.len() {
            if (mask & (1 << i)) != 0 {
                let (l, r) = self.edges[i];
                result ^= (1 << l) ^ (1 << r);
            }
        }
        result == 0 || result.count_ones() == 2
    }

    fn histogram(&self, mask: u64) -> Vec<u8> {
        let mut count = vec![0; self.nodes];
        for i in 0..self.edges.len() {
            if (mask & (1 << i)) != 0 {
                let (l, r) = self.edges[i];
                count[l as usize] += 1;
                count[r as usize] += 1;
            }
        }
        count
    }
}

pub fn run(content: &str) {
    let links = Links::parse(content);
    let result = links.find_best();
    println!("{} {}", result.0, result.1);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

    #[test]
    fn small() {
        let links = super::Links::parse(TEST);
        let exits = vec![vec![4, 3, 2, 0], vec![5, 3, 2, 0], vec![0, 1, 10, 9]];
        assert_eq!(super::Graph::build(&links).exits, exits);
    }
}
