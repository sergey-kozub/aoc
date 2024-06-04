use crate::intcode::IntCode;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn values() -> &'static [Direction; 4] {
        &[Direction::North, Direction::East, Direction::South, Direction::West]
    }

    fn from_str(text: &str) -> Option<Direction> {
        Direction::values().iter().find(|v| v.to_str() == text).copied()
    }

    fn to_str(&self) -> &'static str {
        match *self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west",
        }
    }

    fn next(&self, offset: usize) -> Direction {
        let pos = Direction::values().iter().position(|v| v == self).unwrap();
        Direction::values()[(pos + offset) % 4]
    }
}

#[derive(Clone, Copy, Debug)]
enum Weight {
    Light,
    Heavy,
}

#[derive(Debug)]
struct Room {
    name: String,
    doors: HashMap<Direction, Option<String>>,
    items: HashSet<String>,
}

#[derive(Debug)]
struct Game {
    initial: IntCode,
    cpu: IntCode,
    rooms: HashMap<String, Room>,
    items: HashSet<String>,
    current: String,
    log: Vec<String>,
}

impl Game {
    fn new(program: &str) -> Game {
        let cpu = IntCode::from(program);
        Game {
            initial: cpu.clone(),
            cpu,
            rooms: HashMap::new(),
            items: HashSet::new(),
            current: String::new(),
            log: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.cpu = self.initial.clone();
        self.rooms.clear();
        self.items.clear();
        self.current.clear();
        self.log.clear();
    }

    fn send(&mut self, command: String) {
        for ch in command.as_bytes().iter() {
            self.cpu.input.push_back(*ch as i64);
        }
        if !command.contains('\n') {
            self.cpu.input.push_back(10_i64); // '\n'
        }
        self.log.push(command);
    }

    fn recv(&mut self) -> String {
        let mut buffer = String::new();
        while let Some(v) = self.cpu.wait() {
            buffer.push(v as u8 as char);
        }
        self.log.push(buffer.clone());
        buffer
    }

    fn parse_weight(&self, text: &str) -> Option<Weight> {
        if text.contains("heavier than the detected value") {
            Some(Weight::Light)
        } else if text.contains("lighter than the detected value") {
            Some(Weight::Heavy)
        } else {
            None
        }
    }

    fn parse_room(&self, text: &str) -> Option<Room> {
        let mut name = String::new();
        let mut doors: HashMap<Direction, Option<String>> = HashMap::new();
        let mut items: HashSet<String> = HashSet::new();

        for line in text.lines() {
            if line.starts_with("==") {
                let trimmed = line.trim_matches(|c| c == ' ' || c == '=');
                name = String::from(trimmed);
            }
            if line.starts_with("- ") {
                let trimmed = &line.trim()[2..];
                if let Some(dir) = Direction::from_str(trimmed) {
                    doors.insert(dir, None);
                } else {
                    items.insert(String::from(trimmed));
                }
            }
        }
        if !name.is_empty() {
            Some(Room { name, doors, items })
        } else {
            None
        }
    }

    fn parse_item(&self, text: &str) -> Option<(String, bool)> {
        let re_take = Regex::new(r"You take the (.+?)\.").unwrap();
        if let Some(m) = re_take.captures(text) {
            let name = m.get(1).unwrap().as_str();
            return Some((String::from(name), true));
        }
        let re_drop = Regex::new(r"You drop the (.+?)\.").unwrap();
        if let Some(m) = re_drop.captures(text) {
            let name = m.get(1).unwrap().as_str();
            return Some((String::from(name), false));
        }
        None
    }

    fn next_step(&mut self, text: &str) -> Option<Weight> {
        if let Some(weight) = self.parse_weight(text) {
            return Some(weight);
        }
        if let Some(room) = self.parse_room(text) {
            self.current = room.name.clone();
            match self.rooms.get_mut(&room.name) {
                Some(existing) => {
                    existing.items = room.items;
                },
                None => {
                    self.rooms.insert(room.name.clone(), room);
                },
            }
        }
        if let Some((item, take)) = self.parse_item(text) {
            let room = self.rooms.get_mut(&self.current).unwrap();
            if take {
                room.items.remove(&item);
                self.items.insert(item);
            } else {
                self.items.remove(&item);
                room.items.insert(item);
            }
        }
        None
    }

    fn explore(&mut self, dangerous: &HashSet<String>) -> Result<String, String> {
        let mut item: Option<String> = None;
        let mut step: Option<Direction> = None;
        let mut path: Vec<String> = vec![String::new()];
        let mut checkpoint: Option<String> = None;

        loop {
            let text = self.recv();
            if !self.cpu.is_active() {
                return Result::Err(item.unwrap());
            }

            if let Some(_) = self.next_step(&text) {
                // Weight check failed.
                checkpoint = Some(self.current.clone());
                step = None;
            }

            if let Some(key) = path.last() {
                // Update room references (doors).
                if let Some(dir) = step {
                    let current = self.current.clone();
                    let mut update = |from: &str, to: &str, dir| {
                        let room = self.rooms.get_mut(from).unwrap();
                        room.doors.insert(dir, Some(String::from(to)));
                    };
                    update(&key, &current, dir);
                    update(&current, &key, dir.next(2));
                }

                // Pick up items that are not dangerous.
                let room = self.rooms.get(&self.current).unwrap();
                item = room.items.iter()
                    .filter(|name| !dangerous.contains(*name))
                    .cloned().next();
                if let Some(ref name) = item {
                    self.send(format!("take {}", name));
                    continue;
                }

                // Find next door to use (but skip the checkpoint).
                step = if matches!(checkpoint, Some(ref x) if *x == room.name) {
                    None
                } else {
                    room.doors.iter().find_map(|(k, v)|
                        if v.is_none() {Some(*k)} else {None})
                };

                // Return to the previous room.
                let move_ = step.or_else(|| {
                    room.doors.iter().find_map(|(k, v)|
                        if matches!(v, Some(x) if x == key) {Some(*k)} else {None})
                });
                if let Some(dir) = move_ {
                    self.send(String::from(dir.to_str()));
                }
            } else {
                // Exploration complete.
                return Result::Ok(checkpoint.unwrap());
            }

            match step {
                Some(_) => path.push(self.current.clone()),
                None => { path.pop(); },
            }
        }
    }

    fn find_path(&mut self, target: &str) -> Option<Vec<Direction>> {
        let mut stack: Vec<(String, Vec<Direction>)> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        stack.push((self.current.clone(), Vec::new()));

        while let Some((name, path)) = stack.pop() {
            if name == target { return Some(path); }
            if visited.contains(&name) { continue; }
            let room = self.rooms.get(&name).unwrap();

            for (dir, to) in &room.doors {
                if let Some(to_name) = to {
                    let mut new_path = path.clone();
                    new_path.push(*dir);
                    stack.push((to_name.clone(), new_path));
                }
            }
            visited.insert(name);
        }
        None
    }

    fn try_items(&mut self) {
        let room = self.rooms.get(&self.current).unwrap();
        let dir = room.doors.iter().find_map(|(k, v)|
            if v.is_none() {Some(*k)} else {None}).unwrap();

        let items: Vec<String> = self.items.iter().cloned().collect();
        let mut result: Vec<(HashSet<usize>, Weight)> = Vec::new();

        loop {
            self.send(String::from(dir.to_str()));
            let text = self.recv();
            match self.next_step(&text) {
                Some(weight) => {
                    let set = HashSet::<usize>::from_iter(
                        (0..items.len()).filter(|i| self.items.contains(&items[*i])));
                    result.push((set.clone(), weight));

                    let mut options: Vec<HashSet<usize>> = Vec::new();
                    for packed in 0..(1_u64 << items.len()) {
                        let set = HashSet::<usize>::from_iter(
                            (0..items.len()).filter(|i| packed & 1_u64 << i != 0));
                        if result.iter().all(|(h, w)| {
                            match w {
                                Weight::Light => { !set.is_subset(h) },
                                Weight::Heavy => { !set.is_superset(h) },
                            }
                        }) { options.push(set); }
                    }
                    options.sort_by_key(|v| {
                        -(set.symmetric_difference(v).count() as isize)
                    });

                    let index = options.pop().unwrap();
                    for i in 0..items.len() {
                        if index.contains(&i) != self.items.contains(&items[i]) {
                            self.send(format!("{} {}",
                                if index.contains(&i) {"take"} else {"drop"},
                                items[i]));
                            let text = self.recv();
                            self.next_step(&text);
                        }
                    }
                },
                None => break,
            }
        }
    }

    fn solve(&mut self) {
        let mut dangerous: HashSet<String> = HashSet::new();
        dangerous.insert(String::from("infinite loop"));
        dangerous.insert(String::from("giant electromagnet"));

        loop {
            match self.explore(&dangerous) {
                Result::Ok(checkpoint) => {
                    for dir in self.find_path(&checkpoint).unwrap() {
                        self.send(String::from(dir.to_str()));
                        let text = self.recv();
                        self.next_step(&text);
                    }
                    break;
                },
                Result::Err(item) => {
                    println!(">>> Dangerous item: {}", item);
                    print!("{}", self.log.last().unwrap());
                    dangerous.insert(item);
                    self.reset();
                },
            }
        }

        self.try_items();
        for line in &self.log { print!("{}", line); }
        self.play();
    }

    fn play(&mut self) {
        while self.cpu.is_active() {
            let text = self.recv();
            print!("{}", text);
            self.next_step(&text);

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read input");
            self.send(input);
        }
    }
}

pub fn run(content: &str) {
    Game::new(content).solve();
}
