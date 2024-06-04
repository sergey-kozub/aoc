use crate::intcode::IntCode;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

type Position = (i32, i32);

fn delta(dir: u8) -> Position {
    match dir {
        1 => (0, -1),
        2 => (0, 1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => panic!(),
    }
}

struct Repair {
    cpu: IntCode,
    droid: Position,
    target: Option<Position>,
    mapped: HashMap<Position, bool>,
}

impl Repair {
    fn from(text: &str) -> Repair {
        Repair {
            cpu: IntCode::from(text),
            droid: (0, 0),
            target: None,
            mapped: HashMap::new(),
        }
    }

    fn explore(&mut self) {
        let mut path = vec![self.droid];
        'next: loop {
            for dir in 1..=4 {
                let d = delta(dir);
                let p = (self.droid.0 + d.0, self.droid.1 + d.1);
                if !self.mapped.contains_key(&p) {
                    self.cpu.input.push_back(dir as i64);
                    let res = self.cpu.wait().unwrap();
                    self.mapped.insert(p, res == 0);
                    if res == 2 { self.target = Some(p); }
                    if res != 0 {
                        self.droid = p;
                        path.push(p);
                        continue 'next;
                    }
                }
            }
            path.pop();
            let prev = *path.last().unwrap_or(&self.droid);
            let d = (prev.0 - self.droid.0, prev.1 - self.droid.1);
            match (1..=4).position(|dir| delta(dir) == d) {
                Some(i) => {
                    self.cpu.input.push_back((i + 1) as i64);
                    let res = self.cpu.wait().unwrap();
                    assert_eq!(res, 1);
                    self.droid = prev;
                },
                None => break,
            }
        }
    }

    fn examine(&self) -> (usize, usize) {
        let target = self.target.unwrap();
        let mut heap: BinaryHeap<(isize, Position)> = BinaryHeap::new();
        let mut from: HashMap<Position, Position> = HashMap::new();
        let mut time: isize = 0;

        heap.push((0, target));
        while let Some((n, p)) = heap.pop() {
            if -n > time { time = -n; }
            for dir in 1..=4 {
                let d = delta(dir);
                let next = (p.0 + d.0, p.1 + d.1);
                let can_move = !*self.mapped.get(&next).unwrap_or(&true);
                if can_move && !from.contains_key(&next) {
                    heap.push((n - 1, next));
                    from.insert(next, p);
                }
            }
        }

        let mut path: Vec<Position> = Vec::new();
        let mut pos = self.droid;
        while pos != target {
            path.push(pos);
            pos = *from.get(&pos).unwrap();
        }
        (path.len(), time as usize)
    }
}

impl fmt::Debug for Repair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xmin = *self.mapped.keys().map(|(x, _)| x).min().unwrap();
        let xmax = *self.mapped.keys().map(|(x, _)| x).max().unwrap();
        let ymin = *self.mapped.keys().map(|(_, y)| y).min().unwrap();
        let ymax = *self.mapped.keys().map(|(_, y)| y).max().unwrap();

        let target = self.target.unwrap_or(self.droid);
        let result = (ymin..=ymax).map(|y| -> String {
            (xmin..=xmax).map(|x| {
                if (x, y) == self.droid {'D'}
                else if (x, y) == target {'T'}
                else {
                    match self.mapped.get(&(x, y)) {
                        Some(&v) => if v {'#'} else {'.'},
                        None => ' ',
                    }
                }
            }).collect()
        }).join("\n");
        write!(f, "{}", result)
    }
}

pub fn run(content: &str) {
    let mut inst = Repair::from(content);
    inst.explore();
    println!("{:?}", inst);
    let result = inst.examine();
    println!("{} {}", result.0, result.1);
}
