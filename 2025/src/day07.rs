use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

#[derive(Clone, Debug)]
struct Field {
    splitters: HashSet<Point>,
}

impl Field {
    fn parse(text: &str) -> Self {
        let mut offset = 0;
        let mut splitters = HashSet::new();
        for (row, line) in text.lines().enumerate() {
            if row == 0 {
                offset = line.find('S').unwrap() as i32;
            }
            for (k, v) in line.chars().enumerate() {
                if v == '^' {
                    splitters.insert((k as i32 - offset, row as i32));
                }
            }
        }
        Self { splitters }
    }

    fn run(&self) -> (usize, usize) {
        let limit = self.splitters.iter().map(|&p| p.1).max().unwrap();
        let mut current = HashMap::from([((0, 1), 1_usize)]);
        let mut splits = HashSet::new();
        for _ in 1..=limit {
            let mut next = HashMap::new();
            for ((x, y), c) in current {
                let pts = if self.splitters.contains(&(x, y)) {
                    splits.insert((x, y));
                    vec![(x - 1, y + 1), (x + 1, y + 1)]
                } else {
                    vec![(x, y + 1)]
                };
                for pos in pts {
                    next.entry(pos).and_modify(|e| *e += c).or_insert(c);
                }
            }
            current = next;
        }
        let total = current.values().sum::<usize>();
        (splits.len(), total)
    }
}

pub fn run(content: &str) {
    let (res1, res2) = Field::parse(content).run();
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn small() {
        assert_eq!(super::Field::parse(TEST).run(), (21, 40));
    }
}
