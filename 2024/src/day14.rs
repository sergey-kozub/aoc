use crossterm::{event, terminal};
use std::collections::HashMap;
use std::fmt;

type Coord = (i64, i64);

#[derive(Clone, Debug)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

#[derive(Clone, Debug)]
struct Area {
    robots: Vec<Robot>,
    size: Coord,
}

impl Robot {
    fn parse(text: &str) -> Self {
        let (p, v) = text.split_once(' ').unwrap();
        let to_coord = |s: &str, p: &str| {
            let (l, r) = s.split_once('=').unwrap();
            assert_eq!(l, p);
            let (x, y) = r.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        };
        Self {
            position: to_coord(p, "p"),
            velocity: to_coord(v, "v"),
        }
    }
}

impl Area {
    fn parse(text: &str, size: Coord) -> Self {
        let robots = text.lines().map(Robot::parse).collect();
        Self { robots, size }
    }

    fn advance(&self, steps: i64) -> Self {
        let fix = |a, b| if a < 0 {a + b} else {a};
        let robots = self.robots.iter().map(|r| {
            let x = (r.position.0 + r.velocity.0 * steps) % self.size.0;
            let y = (r.position.1 + r.velocity.1 * steps) % self.size.1;
            let position = (fix(x, self.size.0), fix(y, self.size.1));
            Robot { position, velocity: r.velocity }
        }).collect();
        Self { robots, size: self.size }
    }

    fn quads(&self) -> [i64; 4] {
        let mut quad = [0_i64; 4];
        let (mx, my) = (self.size.0 / 2, self.size.1 / 2);
        for robot in &self.robots {
            let (x, y) = robot.position;
            if x == mx || y == my { continue; }
            let p = if x > mx {1} else {0} + if y > my {2} else {0};
            quad[p] += 1;
        }
        quad
    }

    fn score(&self) -> i64 {
        let q = self.quads();
        q[0] * q[1] * q[2] * q[3]
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = HashMap::new();
        for robot in &self.robots {
            count.entry(robot.position).and_modify(|c| *c += 1).or_insert(1);
        }
        for y in 0..self.size.1 {
            let line = (0..self.size.0).map(|x| {
                match count.get(&(x, y)) {
                    Some(&x) if x <= 9 => (b'0' + x) as char,
                    Some(_) => '*',
                    None => '.',
                }
            }).collect::<String>();
            writeln!(f, "{}\r", line)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn interactive(area: &Area) {
    let mut step = 0_i64;
    terminal::enable_raw_mode().unwrap();
    'main: loop {
        println!("{}step: {}\r", area.advance(step), step);
        step += loop {
            match event::read().unwrap() {
                event::Event::Key(key) => {
                    match key.code {
                        event::KeyCode::Left => break -1,
                        event::KeyCode::Right => break 1,
                        event::KeyCode::Up => break -103,
                        event::KeyCode::Down => break 103,
                        event::KeyCode::Esc => break 'main,
                        _ => continue,
                    }
                },
                _ => continue,
            }
        };
    }
    terminal::disable_raw_mode().unwrap();
}

pub fn run(content: &str) {
    let area = Area::parse(content, (101, 103));
    println!("{}\n{}", area.advance(100).score(), area.advance(6475));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3";

    #[test]
    fn small() {
        let area = super::Area::parse(TEST, (11, 7));
        assert_eq!(area.advance(100).score(), 12);
    }
}
