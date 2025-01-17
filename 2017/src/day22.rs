use std::collections::HashMap;
use std::fmt;

type Position = (i32, i32);
enum Direction { U, R, D, L }

#[derive(PartialEq)]
enum State { Clean, Weakened, Infected, Flagged }

struct Grid {
    active: HashMap<Position, State>,
    current: Position,
    dir: Direction,
    simple: bool,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match &self {
            Direction::U => Direction::R,
            Direction::R => Direction::D,
            Direction::D => Direction::L,
            Direction::L => Direction::U,
        }
    }

    fn turn_left(&self) -> Self {
        self.turn_right().turn_right().turn_right()
    }

    fn turn_back(&self) -> Self {
        self.turn_right().turn_right()
    }
}

impl Grid {
    fn parse(text: &str, simple: bool) -> Self {
        let dy = text.lines().count() as i32 / 2;
        let dx = text.lines().next().unwrap().chars().count() as i32 / 2;
        let active = text.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '.' { return None; }
                let pos = (x as i32 - dx, y as i32 - dy);
                Some((pos, State::Infected))
            })
        }).collect::<HashMap<_, _>>();
        Self { active, current: (0, 0), dir: Direction::U, simple }
    }

    fn step(&mut self) -> bool {
        let cell = self.active.entry(self.current).or_insert(State::Clean);
        *cell = match *cell {
            State::Clean => {
                self.dir = self.dir.turn_left();
                if self.simple {State::Infected} else {State::Weakened}
            },
            State::Weakened => {
                State::Infected
            },
            State::Infected => {
                self.dir = self.dir.turn_right();
                if self.simple {State::Clean} else {State::Flagged}
            },
            State::Flagged => {
                self.dir = self.dir.turn_back();
                State::Clean
            },
        };
        match &self.dir {
            Direction::U => self.current.1 -= 1,
            Direction::R => self.current.0 += 1,
            Direction::D => self.current.1 += 1,
            Direction::L => self.current.0 -= 1,
        }
        *cell == State::Infected
    }

    fn count(self, steps: usize) -> usize {
        self.take(steps).filter(|&x| x).count()
    }
}

impl Iterator for Grid {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        Some(self.step())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (px, py) = self.current;
        let min_x = self.active.keys().map(|x| x.0).min().unwrap().min(px);
        let max_x = self.active.keys().map(|x| x.0).max().unwrap().max(px);
        let min_y = self.active.keys().map(|x| x.1).min().unwrap().min(py);
        let max_y = self.active.keys().map(|x| x.1).max().unwrap().max(py);
        let lines = (min_y..=max_y).map(|y| {
            (min_x..=max_x).map(|x| {
                if self.current == (x, y) {match self.dir {
                    Direction::U => '^',
                    Direction::R => '>',
                    Direction::D => 'v',
                    Direction::L => '<',
                }} else {match self.active.get(&(x, y)) {
                    Some(State::Weakened) => 'W',
                    Some(State::Infected) => '#',
                    Some(State::Flagged) => 'F',
                    _ => '.',
                }}
            }).collect::<String>()
        }).collect::<Vec<_>>();
        write!(f, "{}", lines.join("\n"))
    }
}

pub fn run(content: &str) {
    let v1 = Grid::parse(content, true).count(10_000);
    let v2 = Grid::parse(content, false).count(10_000_000);
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let grid = || super::Grid::parse("..#\n#..\n...", true);
        assert_eq!(grid().count(70), 41);
        assert_eq!(grid().count(10000), 5587);
    }

    #[test]
    fn large() {
        let grid = || super::Grid::parse("..#\n#..\n...", false);
        assert_eq!(grid().count(100), 26);
        //assert_eq!(grid().count(10000000), 2511944);
    }
}
