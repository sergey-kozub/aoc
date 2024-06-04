use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    closest: (i32, i32),
}

impl Sensor {
    fn from(input: &str) -> Sensor {
        let pattern = Regex::new("\
            Sensor at x=(-?\\d+), y=(-?\\d+): \
            closest beacon is at x=(-?\\d+), y=(-?\\d+)"
        ).unwrap();
        let caps = pattern.captures(input).unwrap();
        let get = |i: usize| caps.get(i).unwrap().as_str()
            .parse::<i32>().unwrap();
        Sensor {
            position: (get(1), get(2)),
            closest: (get(3), get(4)),
        }
    }

    fn distance(&self) -> i32 {
        (self.closest.0 - self.position.0).abs() +
        (self.closest.1 - self.position.1).abs()
    }
    
    fn at_row(&self, row: i32) -> Option<RangeInclusive<i32>> {
        let (x, y) = self.position;
        let width = self.distance() - (row - y).abs();
        if width >= 0 {
            Some(RangeInclusive::new(x - width, x + width))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Grid {
    sensors: Vec<Sensor>,
}

impl Grid {
    fn from(input: &str) -> Grid {
        let sensors: Vec<Sensor> = input.lines().map(Sensor::from).collect();
        Grid { sensors }
    }

    fn at_row(&self, row: i32) -> Vec<RangeInclusive<i32>> {
        let mut result = Vec::<RangeInclusive<i32>>::new();
        for it in &self.sensors {
            if let Some(r) = it.at_row(row) {
                let left = result.iter().position(
                    |x| r.start() <= x.end()).unwrap_or(result.len());
                let right = result.iter().position(
                    |x| r.end() < x.start()).unwrap_or(result.len());
                if left < right {
                    let nr = RangeInclusive::<i32>::new(
                        min(*result[left].start(), *r.start()),
                        max(*result[right - 1].end(), *r.end()));
                    result.splice(left..right, vec![nr]);
                } else {
                    result.insert(left, r);
                }
            }
        }
        result
    }
    
    fn count(&self, row: i32) -> i32 {
        self.at_row(row).iter().map(|r| r.end() - r.start() + 1).sum::<i32>() -
        HashSet::<i32>::from_iter(self.sensors.iter().filter_map(|s| {
            if s.closest.1 == row {Some(s.closest.0)} else {None}
        })).len() as i32
    }

    fn find(&self, row_range: RangeInclusive<i32>) -> Option<(i32, i32)> {
        for row in row_range {
            for a in self.at_row(row).windows(2) {
                if a[0].end() + 2 == *a[1].start() {
                    return Some((a[0].end() + 1, row));
                }
            }
        }
        None
    }
}

pub fn run(content: &str) {
    let grid = Grid::from(content);
    let count = grid.count(2_000_000);
    let (x, y) = grid.find(0..=4_000_000).unwrap();
    let score = x as i64 * 4_000_000 + y as i64;
    println!("{} {}", count, score);
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { "\
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
        Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
        Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
        Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
        Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
        Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
        Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
        Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
        Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
        Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
        Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
        Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
        Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
        Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }

    #[test]
    pub fn beacon() {
        let grid = super::Grid::from(example());
        assert_eq!(grid.count(10), 26);
        assert_eq!(grid.find(0..=20).unwrap(), (14, 11));
    }
}
