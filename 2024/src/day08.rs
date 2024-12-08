use std::collections::{HashMap, HashSet};

type Position = (i32, i32);

struct Grid {
    antenna: HashMap<char, Vec<Position>>,
    size: (i32, i32),
}

impl Grid {
    fn parse(text: &str) -> Self {
        let mut antenna = HashMap::new();
        let mut size = (0, 0);
        for (y, line) in text.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch.is_ascii_alphanumeric() {
                    let pt = (x as i32, y as i32);
                    antenna.entry(ch).or_insert(vec![]).push(pt);
                }
            }
            size = (line.len() as i32, y as i32 + 1)
        }
        Self { antenna, size }
    }

    fn antinode(&self, repeat: bool) -> HashSet<Position> {
        let mut result = HashSet::new();
        let in_bounds = |(x, y)|
            x >= 0 && x < self.size.0 && y >= 0 && y < self.size.1;
        let (l, r) = if repeat {(0, 100)} else {(1, 1)};
        for group in self.antenna.values() {
            for i in 1..group.len() {
                for j in 0..i {
                    let (p1, p2) = (group[i], group[j]);
                    let (dx, dy) = (p2.0 - p1.0, p2.1 - p1.1);
                    for k in l..=r as i32 {
                        let pt = (p1.0 - dx * k, p1.1 - dy * k);
                        if !in_bounds(pt) { break; }
                        result.insert(pt);
                    }
                    for k in l..=r as i32 {
                        let pt = (p2.0 + dx * k, p2.1 + dy * k);
                        if !in_bounds(pt) { break; }
                        result.insert(pt);
                    }
                }
            }
        }
        result
    }
}

pub fn run(content: &str) {
    let grid = Grid::parse(content);
    println!("{} {}", grid.antinode(false).len(), grid.antinode(true).len());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............";

    #[test]
    fn small() {
        let grid = super::Grid::parse(TEST);
        assert_eq!(grid.antinode(false).len(), 14);
    }

    #[test]
    fn large() {
        let grid = super::Grid::parse(TEST);
        assert_eq!(grid.antinode(true).len(), 34);
    }
}
