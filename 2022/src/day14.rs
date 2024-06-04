use itertools::Itertools;
use std::collections::HashSet;

const START_X: i32 = 500;
const START_Y: i32 = 0;

#[derive(Clone)]
struct Maze {
    walls: HashSet<(i32, i32)>,
    ground: i32,
}

impl Maze {
    fn from(input: &str) -> Maze {
        let mut walls = HashSet::<(i32, i32)>::new();
        let mut ground = START_Y;
        let range = |a: i32, b: i32| if a < b {a..=b} else {b..=a};
        for line in input.lines() {
            let points: Vec<(i32, i32)> = line.split(" -> ").map(|s|
                s.split(',').map(|x| x.parse::<i32>().unwrap())
                    .collect_tuple().unwrap()
            ).collect();
            for a in points.windows(2) {
                if a[0].0 == a[1].0 {
                    walls.extend(range(a[0].1, a[1].1).map(|i| (a[0].0, i)));
                } else if a[0].1 == a[1].1 {
                    walls.extend(range(a[0].0, a[1].0).map(|i| (i, a[0].1)));
                } else {
                    panic!("Incorrect input: {a:?}");
                }
            }
            let lowest = points.iter().map(|(_, y)| *y).max().unwrap();
            if lowest > ground { ground = lowest; }
        }
        Maze { walls, ground }
    }

    #[allow(dead_code)]
    fn print(&self, pos: (i32, i32), size: (usize, usize)) {
        for y in 0..size.1 {
            for x in 0..size.0 {
                let p = (pos.0 + x as i32, pos.1 + y as i32);
                print!("{}", if self.walls.contains(&p) {'#'} else {'.'});
            }
            println!();
        }
    }

    fn drop(&self) -> (i32, i32) {
        let mut x = START_X;
        let mut y = START_Y;
        while y <= self.ground {
            let next = [x, x - 1, x + 1].into_iter().filter(|i|
                !self.walls.contains(&(*i, y + 1))).next();
            match next {
                Some(i) => { x = i; y += 1; },
                None => break,
            }
        }
        (x, y)
    }

    fn count(&mut self, has_floor: bool) -> usize {
        let mut count = has_floor as usize;
        loop {
            let pos = self.drop();
            let end = if has_floor {
                pos == (START_X, START_Y)
            } else {
                pos.1 > self.ground
            };
            if end { break; }
            self.walls.insert(pos);
            count += 1;
        }
        count
    }
}

pub fn run(content: &str) {
    let maze = Maze::from(content);
    println!("{} {}", maze.clone().count(false), maze.clone().count(true));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn sand() {
        let maze = super::Maze::from("\
            498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9");
        assert_eq!(maze.clone().count(false), 24);
        assert_eq!(maze.clone().count(true), 93);
    }
}
