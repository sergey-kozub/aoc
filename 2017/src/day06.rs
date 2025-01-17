use std::collections::HashSet;

struct MemoryBanks {
    blocks: Vec<usize>,
    visited: HashSet<String>,
}

impl MemoryBanks {
    fn new(blocks: Vec<usize>) -> Self {
        let s = Self::footprint(&blocks);
        Self { blocks, visited: HashSet::from([s]) }
    }

    fn footprint(data: &[usize]) -> String {
        data.iter().map(|&v| v.to_string())
            .collect::<Vec<_>>().join(":")
    }

    fn select_max(&self) -> (usize, usize) {
        let (mut max_k, mut max_v) = (0, 0);
        for i in 0..self.blocks.len() {
            if self.blocks[i] > max_v {
                (max_k, max_v) = (i, self.blocks[i]);
            }
        }
        (max_k, max_v)
    }

    fn find_loop(mut self) -> (usize, Self) {
        let mut count = 0;
        while let Some(_) = self.next() { count += 1; }
        (count + 1, Self::new(self.blocks))
    }
}

impl Iterator for MemoryBanks {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        let (k, v) = self.select_max();
        self.blocks[k] = 0;
        let n = self.blocks.len();
        for i in 1..=v {
            self.blocks[(k + i) % n] += 1;
        }
        let s = Self::footprint(&self.blocks);
        if self.visited.insert(s) {Some(())} else {None}
    }
}

pub fn run(content: &str) {
    let data = content.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let iter = MemoryBanks::new(data);
    let (count, iter) = iter.find_loop();
    println!("{} {}", count, iter.find_loop().0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let iter = super::MemoryBanks::new(vec![0, 2, 7, 0]);
        assert_eq!(iter.find_loop().0, 5);
    }

    #[test]
    fn large() {
        let iter = super::MemoryBanks::new(vec![0, 2, 7, 0]);
        let (_, iter) = iter.find_loop();
        assert_eq!(iter.find_loop().0, 4);
    }
}
