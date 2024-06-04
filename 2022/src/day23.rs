use std::collections::{HashMap, HashSet};
use std::fmt;

type Coord = (i32, i32);

#[derive(Clone, Copy)]
enum Direction { North, South, West, East }

struct Party {
    elves: HashSet<Coord>,
    dirs: Vec<Direction>,
    steps: usize,
}

impl Party {
    fn from(input: &str) -> Party {
        let mut elves = HashSet::<Coord>::new();
        for (y, row) in input.lines().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' { elves.insert((x as i32, y as i32)); }
            }
        }
        let dirs = Vec::from([
            Direction::North, Direction::South, Direction::West, Direction::East
        ]);
        Party { elves, dirs, steps: 0 }
    }

    fn check(&self, (x, y): Coord, dir: Direction) -> Option<Coord> {
        let pts = match dir {
            Direction::North => [(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)],
            Direction::South => [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)],
            Direction::West => [(x - 1, y), (x - 1, y - 1), (x - 1, y + 1)],
            Direction::East => [(x + 1, y), (x + 1, y - 1), (x + 1, y + 1)],
        };
        if pts.iter().all(|p| !self.elves.contains(p)) {
            Some(pts[0])
        } else {
            None
        }
    }

    fn spread(&mut self) -> bool {
        let mut new = HashMap::<Coord, Vec<Coord>>::new();
        for pos in &self.elves {
            let m = Vec::from_iter(self.dirs.iter().filter_map(
                |&d| self.check(*pos, d)));
            let to = if !m.is_empty() && m.len() != 4 {m[0]} else {*pos};
            new.entry(to).or_insert(Vec::new()).push(*pos);
        }
        let new_state = HashSet::<Coord>::from_iter(
            new.into_iter().flat_map(|(k, v)| {
                (if v.len() > 1 {v} else {vec![k]}).into_iter()
            }));
        if new_state == self.elves { return false; }
        self.elves = new_state;
        let dir = self.dirs.remove(0);
        self.dirs.push(dir);
        self.steps += 1;
        true
    }

    fn get_bounds(&self) -> (Coord, Coord) {
        let x_min = self.elves.iter().map(|&p| p.0).min().unwrap();
        let x_max = self.elves.iter().map(|&p| p.0).max().unwrap();
        let y_min = self.elves.iter().map(|&p| p.1).min().unwrap();
        let y_max = self.elves.iter().map(|&p| p.1).max().unwrap();
        ((x_min, y_min), (x_max, y_max))
    }

    fn score(&self) -> usize {
        let ((x_min, y_min), (x_max, y_max)) = self.get_bounds();
        let area = (y_max - y_min + 1) * (x_max - x_min + 1);
        area as usize - self.elves.len()
    }
}

impl fmt::Display for Party {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ((x_min, y_min), (x_max, y_max)) = self.get_bounds();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let empty = !self.elves.contains(&(x, y));
                write!(f, "{}", if empty {'.'} else {'#'})?;
            }
            writeln!(f)?;
        }
        fmt::Result::Ok(())
    }
}

pub fn run(content: &str) {
    let mut inst = Party::from(content);
    for _ in 0..10 { inst.spread(); }
    let score = inst.score();
    while inst.spread() {}
    println!("{} {}", score, inst.steps + 1);
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { "
        ....#..\n\
        ..###.#\n\
        #...#.#\n\
        .#...##\n\
        #.###..\n\
        ##.#.##\n\
        .#..#..".trim()
    }

    #[test]
    pub fn elves() {
        let mut inst = super::Party::from(example());
        for _ in 0..10 { inst.spread(); }
        assert_eq!(inst.score(), 110);
        while inst.spread() {}
        assert_eq!(inst.steps, 19);
    }
}
