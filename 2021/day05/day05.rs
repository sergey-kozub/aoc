use std::{cmp, fs, ops};
use std::collections::HashMap;

type Point = (i32, i32);

#[derive(Debug)]
struct Area {
    start: Point,
    end: Point,
}

impl Area {
    fn create(text: &str) -> Area {
        let parse = |s: &str| -> Point {
            let mut it = s.split(',').map(|x| x.parse::<i32>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        };
        let parts: Vec<&str> = text.split(" -> ").collect();
        Area {
            start: parse(parts[0]),
            end: parse(parts[1]),
        }
    }

    fn is_vertical(&self) -> bool { self.start.0 == self.end.0 }
    fn is_horizontal(&self) -> bool { self.start.1 == self.end.1 }

    fn x_min(&self) -> i32 { cmp::min(self.start.0, self.end.0) }
    fn x_max(&self) -> i32 { cmp::max(self.start.0, self.end.0) }
    fn y_min(&self) -> i32 { cmp::min(self.start.1, self.end.1) }
    fn y_max(&self) -> i32 { cmp::max(self.start.1, self.end.1) }

    fn x_iter(&self) -> ops::RangeInclusive<i32> { self.x_min() ..= self.x_max() }
    fn y_iter(&self) -> ops::RangeInclusive<i32> { self.y_min() ..= self.y_max() }

    fn get_points(&self) -> Vec<Point> {
        if self.is_vertical() {
            self.y_iter().map(|y| (self.start.0, y)).collect()
        } else
        if self.is_horizontal() {
            self.x_iter().map(|x| (x, self.start.1)).collect()
        } else {
            let x_increasing = self.start.0 < self.end.0;
            let y_increasing = self.start.1 < self.end.1;
            let y_list: Vec<i32> = if x_increasing == y_increasing {
                self.y_iter().collect()
            } else {
                self.y_iter().rev().collect()
            };
            let mut y_iter = y_list.iter();
            self.x_iter().map(|x| (x, *y_iter.next().unwrap())).collect()
        }
    }
}

fn main() {
    let input: Vec<Area> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(Area::create).collect();

    let mut map_hv: HashMap<Point, i32> = HashMap::new();
    let mut map_all: HashMap<Point, i32> = HashMap::new();
    for area in input {
        let is_hv = area.is_horizontal() || area.is_vertical();
        for point in area.get_points() {
            if is_hv { *map_hv.entry(point).or_insert(0) += 1 };
            *map_all.entry(point).or_insert(0) += 1
        }
    }

    let score = |x: &HashMap<_, i32>| -> usize { x.values().filter(|&x| *x > 1).count() };
    println!("{} {}", score(&map_hv), score(&map_all));
}
