use std::collections::{HashMap, HashSet};
use std::fmt;

type Count = Vec<u32>;
type State = HashMap<Shape, HashSet<Count>>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Shape {
    size: (i32, i32),
    empty: u32,
    field: Vec<u8>,
}

#[derive(Clone, Debug)]
struct Task {
    size: (i32, i32),
    count: Count,
}

#[derive(Clone, Debug)]
struct Problem {
    shapes: Vec<Shape>,
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
struct Limits {
    width: i32,
    height: i32,
    empty: u32,
}

impl Shape {
    fn width(&self) -> i32 { self.size.0 }
    fn height(&self) -> i32 { self.size.1 }
    fn area(&self) -> i32 { self.width() * self.height() }

    fn get(&self, x: i32, y: i32) -> u8 {
        self.field[(y * self.size.0 + x) as usize]
    }

    fn rotate_1(&self) -> Self {
        let field = (0..self.width()).flat_map(|x| {
            (0..self.height()).rev().map(move |y| self.get(x, y))
        }).collect::<Vec<_>>();
        let size = (self.height(), self.width());
        Self { size, field, empty: self.empty }
    }

    fn rotate_2(&self) -> Self {
        let field = (0..self.height()).rev().flat_map(|y| {
            (0..self.width()).rev().map(move |x| self.get(x, y))
        }).collect::<Vec<_>>();
        Self { size: self.size, field, empty: self.empty }
    }

    fn rotate_3(&self) -> Self {
        let field = (0..self.width()).rev().flat_map(|x| {
            (0..self.height()).map(move |y| self.get(x, y))
        }).collect::<Vec<_>>();
        let size = (self.height(), self.width());
        Self { size, field, empty: self.empty }
    }

    fn flip_h(&self) -> Self {
        let field = (0..self.height()).flat_map(|y| {
            (0..self.width()).rev().map(move |x| self.get(x, y))
        }).collect::<Vec<_>>();
        Self { size: self.size, field, empty: self.empty }
    }

    fn flip_v(&self) -> Self {
        let field = (0..self.height()).rev().flat_map(|y| {
            (0..self.width()).map(move |x| self.get(x, y))
        }).collect::<Vec<_>>();
        Self { size: self.size, field, empty: self.empty }
    }

    fn all_mods(&self) -> HashSet<Self> {
        let mut result = HashSet::from([self.clone()]);
        result.insert(self.rotate_1());
        result.insert(self.rotate_2());
        result.insert(self.rotate_3());
        result.insert(self.flip_h());
        result.insert(self.flip_v());
        result
    }

    fn safe_get(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x < self.width() && y >= 0 && y < self.height() {
            self.get(x, y)
        } else {0}
    }

    fn join(&self, other: &Self, pos: (i32, i32)) -> Option<Self> {
        for y in 0..other.height() {
            for x in 0..other.width() {
                if other.get(x, y) != 0 &&
                    self.safe_get(pos.0 + x, pos.1 + y) != 0 { return None; }
            }
        }
        let l = (-pos.0).max(0);
        let r = (pos.0 + other.width() - self.width()).max(0);
        let u = (-pos.1).max(0);
        let d = (pos.1 + other.height() - self.height()).max(0);
        let (w, h) = (self.width() + l + r, self.height() + u + d);

        let mut field = vec![0_u8; (w * h) as usize];
        for y in 0..self.height() {
            for x in 0..self.width() {
                let idx = (y + u) * w + (x + l);
                field[idx as usize] = self.get(x, y);
            }
        }
        for y in 0..other.height() {
            for x in 0..other.width() {
                let idx = (pos.1 + y + u) * w + (pos.0 + x + l);
                field[idx as usize] += other.get(x, y);
            }
        }

        let filled = (self.area() as u32 - self.empty) +
            (other.area() as u32 - other.empty);
        let empty = (w * h) as u32 - filled;
        Some(Self { size: (w, h), field, empty })
    }

    fn join_all(&self, other: &Self, edge_h: i32, edge_v: i32) -> Vec<Self> {
        let dx = self.width() - other.width();
        let dy = self.height() - other.height();
        let mut result = vec![];
        if edge_v != 0 {
            for x in -edge_h..dx + edge_h.min(1) {
                for y in [-edge_v, dy + edge_v] {
                    if let Some(v) = self.join(other, (x, y)) {
                        result.push(v);
                    }
                }
            }
        }
        if edge_h != 0 {
            for y in -edge_v..dy + edge_v.min(1) {
                for x in [-edge_h, dx + edge_h] {
                    if let Some(v) = self.join(other, (x, y)) {
                        result.push(v);
                    }
                }
            }
        }
        result
    }
}

fn update(c1: &HashSet<Count>, c2: &HashSet<Count>) -> HashSet<Count> {
    let mut result = HashSet::new();
    for a1 in c1 {
        for a2 in c2 {
            let count = a1.iter().zip(a2.iter()).map(|(a, b)| a + b)
                .collect::<Vec<_>>();
            result.insert(count);
        }
    }
    result
}

fn grow(large: &State, small: &State, limits: &Limits) -> State {
    let mut result = State::new();
    for (s1, c1) in large {
        let max_h = (limits.width - s1.width()).min(2);
        let max_v = (limits.height - s1.height()).min(2);
        for edge_h in 0..=max_h {
            for edge_v in 0..=max_v {
                if edge_h == 0 && edge_v == 0 { continue; }
                for (s2, c2) in small {
                    for shape in s1.join_all(s2, edge_h, edge_v) {
                        if shape.empty > limits.empty { continue; }
                        result.entry(shape)
                            .and_modify(|v| v.extend(update(c1, c2)))
                            .or_insert_with(|| update(c1, c2));
                    }
                }
            }
        }
    }
    result
}

fn grow_upto(small: &State, limits: &Limits) -> State {
    let mut result = State::new();
    let mut current = small.clone();
    loop {
        let next = grow(&current, small, limits);
        // println!("{}", next.len());
        if next.is_empty() { return result; }
        current = next;
        for (k, v) in current.clone() {
            result.entry(k)
                .and_modify(|x| *x = update(x, &v))
                .or_insert(v);
        }
    }
}

fn _search(state: &State, task: &Task) -> bool {
    state.iter().any(|(k, v)| {
        k.width() <= task.size.0 && k.height() <= task.size.1 &&
        v.contains(&task.count)
    })
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            let line = (0..self.width())
                .map(|x| if self.get(x, y) != 0 {'#'} else {'.'})
                .collect::<String>();
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

impl Problem {
    fn parse(text: &str) -> Self {
        let parts = text.split("\n\n").collect::<Vec<_>>();
        let shapes = parts.iter().take(parts.len() - 1).map(|part| {
            let lines = part.lines().skip(1).collect::<Vec<_>>();
            let (w, h) = (lines[0].len() as i32, lines.len() as i32);
            let field = lines.into_iter().flat_map(|line| {
                line.chars().map(|c| if c == '#' {1} else {0})
            }).collect::<Vec<_>>();
            let empty = field.iter().filter(|&&x| x == 0).count() as u32;
            Shape { size: (w, h), field, empty }
        }).collect::<Vec<_>>();
        let tasks = parts.last().unwrap().lines().map(|s| {
            let as_int = |v: &str| v.parse::<i32>().unwrap();
            let (l, r) = s.split_once(": ").unwrap();
            let (w, h) = l.split_once("x").unwrap();
            let count = r.split(" ").map(|t| as_int(t) as u32)
                .collect::<Vec<_>>();
            Task { size: (as_int(w), as_int(h)), count }
        }).collect::<Vec<_>>();
        Self { shapes, tasks }
    }

    fn initial(&self) -> State {
        let mut result = State::new();
        for (index, shape) in self.shapes.iter().enumerate() {
            let mut count = vec![0; self.shapes.len()];
            count[index] = 1;
            for mod_ in shape.all_mods() {
                result.entry(mod_)
                    .and_modify(|v| { v.insert(count.clone()); })
                    .or_insert_with(|| HashSet::from([count.clone()]));
            }
        }
        result
    }
}

const _TEST: &str = "\
    0:\n###\n##.\n##.\n\n\
    1:\n###\n##.\n.##\n\n\
    2:\n.##\n###\n##.\n\n\
    3:\n##.\n###\n##.\n\n\
    4:\n###\n#..\n###\n\n\
    5:\n###\n.#.\n###\n\n\
    4x4: 0 0 0 0 2 0\n\
    12x5: 1 0 1 0 2 2\n\
    12x5: 1 0 1 0 3 2";

pub fn run(content: &str) {
    let problem = Problem::parse(content);
    let limits = Limits { width: 50, height: 50, empty: 0 };
    let _ = grow_upto(&problem.initial(), &limits);

    let mut count = 0;
    for task in &problem.tasks {
        let filled = task.count.iter().enumerate().map(|(k, v)| {
            let s = &problem.shapes[k];
            (s.area() as u32 - s.empty) * v
        }).sum::<u32>();
        let area = (task.size.0 * task.size.1) as u32;
        if filled < area { count += 1; }
    }
    println!("{count}");
}
