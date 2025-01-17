use crate::day10::knot_hash;
use std::collections::HashSet;

struct Grid(Vec<Vec<u8>>);

impl Grid {
    fn build(text: &str) -> Self {
        let data = (0..128).map(|i| {
            let s = knot_hash(&format!("{text}-{i}"));
            s.into_bytes().into_iter().flat_map(|c| {
                let x = if c >= b'a' {c - b'a' + 10} else {c - b'0'};
                (0..4).rev().map(move |b| (x >> b) & 1)
            }).collect()
        }).collect();
        Self(data)
    }

    fn count_bits(&self) -> usize {
        self.0.iter().flat_map(|x| x.iter())
            .map(|&x| x as usize).sum()
    }

    fn count_groups(&self) -> usize {
        let mut groups = 0;
        let mut bits = self.0.iter().enumerate().flat_map(|(y, a)| {
            a.iter().enumerate()
                .filter(|(_, v)| **v != 0)
                .map(move |(x, _)| (x as isize, y as isize))
        }).collect::<HashSet<_>>();
        while !bits.is_empty() {
            let start = bits.iter().next().cloned().unwrap();
            bits.remove(&start);
            let mut queue = vec![start];
            while let Some((x, y)) = queue.pop() {
                for pos in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if bits.remove(&pos) { queue.push(pos); }
                }
            }
            groups += 1;
        }
        groups
    }
}

pub fn run(content: &str) {
    let grid = Grid::build(content);
    println!("{} {}", grid.count_bits(), grid.count_groups());
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::Grid::build("flqrgnkx").count_bits(), 8108);
    }

    #[test]
    fn large() {
        assert_eq!(super::Grid::build("flqrgnkx").count_groups(), 1242);
    }
}
