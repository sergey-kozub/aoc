use std::collections::HashMap;

enum Segment {
    Letter(char),
    Vertical,
    Horizontal,
    Turn,
}

type Position = (i32, i32);
enum Direction { U, R, D, L }

struct Path {
    segments: HashMap<Position, Segment>,
    current: Position,
    dir: Direction,
    letters: String,
}

impl Path {
    fn parse(text: &str) -> Self {
        let mut segments = HashMap::new();
        for (y, line) in text.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let item = match ch {
                    'A'..='Z' => Segment::Letter(ch),
                    '|' => Segment::Vertical,
                    '-' => Segment::Horizontal,
                    '+' => Segment::Turn,
                    ' ' => continue,
                    _ => panic!(),
                };
                segments.insert((x as i32, y as i32), item);
            }
        }
        let current = *segments.keys().min_by_key(|(x, y)| (y, x)).unwrap();
        Self { segments, current, dir: Direction::D, letters: String::new() }
    }

    fn forward(&self, dist: i32) -> Position {
        let (x, y) = self.current;
        match self.dir {
            Direction::U => (x, y - dist),
            Direction::R => (x + dist, y),
            Direction::D => (x, y + dist),
            Direction::L => (x - dist, y),
        }
    }
}

impl Iterator for Path {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        let (x, y) = self.forward(1);
        match self.segments.get(&(x, y))? {
            Segment::Letter(ch) => self.letters.push(*ch),
            Segment::Vertical | Segment::Horizontal => {},
            Segment::Turn => match self.dir {
                Direction::U | Direction::D => {
                    self.dir = match self.segments.get(&(x - 1, y)) {
                        Some(Segment::Letter(_) | Segment::Horizontal |
                             Segment::Turn) => Direction::L,
                        _ => Direction::R,
                    };
                },
                Direction::L | Direction::R => {
                    self.dir = match self.segments.get(&(x, y - 1)) {
                        Some(Segment::Letter(_) | Segment::Vertical |
                             Segment::Turn) => Direction::U,
                        _ => Direction::D,
                    }
                },
            },
        }
        self.current = (x, y);
        Some(())
    }
}

pub fn run(content: &str) {
    let mut path = Path::parse(content);
    let mut count = 1;
    while path.next().is_some() { count += 1; }
    println!("{} {}", path.letters, count);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "
        |
        |  +--+
        A  |  C
    F---|----E|--+
        |  |  |  D
        +B-+  +--+";

    #[test]
    fn small() {
        let mut path = super::Path::parse(TEST);
        let mut count = 1;
        while path.next().is_some() { count += 1; }
        assert_eq!(path.letters, "ABCDEF");
        assert_eq!(count, 38);
    }
}
