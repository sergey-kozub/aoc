
enum Direction { N, NE, NW, S, SE, SW }

impl Direction {
    fn parse(text: &str) -> Self {
        match text {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "nw" => Direction::NW,
            "s" => Direction::S,
            "se" => Direction::SE,
            "sw" => Direction::SW,
            _ => panic!(),
        }
    }

    fn distance(text: &str) -> (usize, usize) {
        let (mut x, mut y) = (0_isize, 0_isize);
        let mut max_dist = 0;
        for s in text.split(",") {
            let odd = x.abs() % 2;
            (x, y) = match Direction::parse(s) {
                Direction::N => (x, y + 1),
                Direction::NE => (x + 1, y + odd),
                Direction::NW => (x - 1, y + odd),
                Direction::S => (x, y - 1),
                Direction::SE => (x + 1, y - 1 + odd),
                Direction::SW => (x - 1, y - 1 + odd),
            };
            max_dist = max_dist.max(calc(x.abs(), y));
        }
        (calc(x.abs(), y) as usize, max_dist as usize)
    }
}

fn calc(x: isize, y: isize) -> isize {
    if x == 0 || y == 0 { return x + y.abs(); }
    1 + calc(x - 1, y - (y > 0) as isize + x % 2)
}

pub fn run(content: &str) {
    let (a, b) = Direction::distance(content);
    println!("{} {}", a, b);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let distance = |s| super::Direction::distance(s).0;
        assert_eq!(distance("ne,ne,ne"), 3);
        assert_eq!(distance("ne,ne,sw,sw"), 0);
        assert_eq!(distance("ne,ne,s,s"), 2);
        assert_eq!(distance("se,sw,se,sw,sw"), 3);
    }
}
