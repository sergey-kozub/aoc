use std::collections::HashSet;

enum Move {
    Left(i64),
    Right(i64),
}

impl Move {
    fn parse(path: &str) -> Vec<Self> {
        path.split(", ").map(|s| {
            let (l, r) = s.split_at(1);
            match l {
                "L" => Move::Left(r.parse().unwrap()),
                "R" => Move::Right(r.parse().unwrap()),
                _ => panic!(),
            }
        }).collect()
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

struct State {
    x: i64,
    y: i64,
    dir: Direction,
}

impl State {
    fn step(&mut self, n: i64) {
        match self.dir {
            Direction::North => self.y += n,
            Direction::South => self.y -= n,
            Direction::East => self.x += n,
            Direction::West => self.x -= n,
        }
    }

    fn go(&mut self, to: &Move) {
        match to {
            Move::Left(n) => {
                self.dir = self.dir.turn().turn().turn();
                self.step(*n);
            },
            Move::Right(n) => {
                self.dir = self.dir.turn();
                self.step(*n);
            },
        }
    }
}

fn walk_distance(path: &str) -> i64 {
    let mut state = State { x: 0, y: 0, dir: Direction::North };
    for to in Move::parse(path) {
        state.go(&to);
    }
    state.x.abs() + state.y.abs()
}

fn find_cross(path: &str) -> i64 {
    let mut state = State { x: 0, y: 0, dir: Direction::North };
    let mut visited = HashSet::<(i64, i64)>::new();
    visited.insert((0, 0));
    for to in Move::parse(path) {
        let step = match to {
            Move::Left(n) => { state.dir = state.dir.turn().turn().turn(); n },
            Move::Right(n) => { state.dir = state.dir.turn(); n },
        };
        for _ in 0..step {
            state.step(1);
            let pt = (state.x, state.y);
            if visited.contains(&pt) { return pt.0 + pt.1; }
            visited.insert(pt);
        }
    }
    i64::MAX
}

pub fn run(content: &str) {
    println!("{} {}", walk_distance(content), find_cross(content));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::walk_distance("R2, L3"), 5);
        assert_eq!(super::walk_distance("R2, R2, R2"), 2);
        assert_eq!(super::walk_distance("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn large() {
        assert_eq!(super::find_cross("R8, R4, R4, R8"), 4);
    }
}
