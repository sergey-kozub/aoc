use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::fs;

const ROOMS: usize = 4;
const EDGE: usize = 2;
const HALLWAY: usize = (ROOMS + EDGE) * 2 - 1;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Field(Vec<u8>);

type Position = (usize, usize);
type Move = (Position, Position);

fn addr(room: usize, level: usize) -> usize { HALLWAY + ROOMS * (level - 1) + room }
fn conv(room: usize) -> usize { room * 2 + EDGE }
fn skip(i: usize) -> bool { (0..ROOMS).any(|j| i == conv(j)) }
fn index(p: Position) -> usize { if p.1 > 0 {addr((p.0 - EDGE) / 2, p.1)} else {p.0} }

impl Field {
    fn from(text: &str) -> Field {
        let mut data = vec![b'.'; HALLWAY];
        text.chars().for_each(|c| {
            if c.is_ascii_alphabetic() {
                data.push(c as u8);
            }
        });
        Field(data)
    }

    fn size(&self) -> usize {
        (self.0.len() - HALLWAY) / ROOMS
    }

    fn is_final(&self) -> bool {
        for i in 0..ROOMS { for j in 1..=self.size() {
            if self.0[addr(i, j)] != b"ABCD"[i] {
                return false;
            }
        }}
        (0..HALLWAY).all(|i| self.0[i] == b'.')
    }

    fn get_top(&self, room: usize) -> Option<usize> {
        for i in 1..=self.size() {
            let value = self.0[addr(room, i)];
            if value != b'.' { return Some(i); }
        }
        None
    }

    fn valid_target(&self, room: usize) -> bool {
        match self.get_top(room) {
            Some(top) => top > 1 && (top..=self.size()).all(|i|
                self.0[addr(room, i)] == b"ABCD"[room]),
            None => true,
        }
    }

    fn can_reach(&self, from: Position, to: Position) -> bool {
        if from == to { return true; }
        if self.0[index(from)] != b'.' { return false; }
        if from.1 > 0 || from.0 == to.0 {
            let y = from.1 as isize + if from.0 == to.0 {1} else {-1};
            self.can_reach((from.0, y as usize), to)
        } else {
            let x = from.0 as isize + if from.0 < to.0 {1} else {-1};
            self.can_reach((x as usize, 0), to)
        }
    }

    fn find_moves(&self) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let mut try_move = |a, b| if self.can_reach(b, a) { result.push((a, b)) };

        for i in 0..HALLWAY { if self.0[i] != b'.' {
            //for j in 0..HALLWAY { if self.0[j] == b'.' && !skip(j) {
            //    try_move((i, 0), (j, 0));
            //}}
            let room = "ABCD".find(self.0[i] as char).unwrap();
            if self.valid_target(room) {
                try_move((i, 0), match self.get_top(room) {
                    Some(top) => (conv(room), top - 1),
                    None => (conv(room), self.size()),
                });
            }
        }}
        for i in 0..ROOMS {
            if let Some(top) = self.get_top(i) {
                for j in 0..HALLWAY { if self.0[j] == b'.' && !skip(j) {
                    try_move((conv(i), top), (j, 0));
                }}
            }
        }
        result
    }

    fn apply_move(&self, (from, to): Move) -> (Field, i64) {
        let mut field = (*self).clone();
        let letter = field.0[index(from)];
        field.0[index(from)] = b'.';
        field.0[index(to)] = letter;

        let dist = (from.0 as isize - to.0 as isize).abs() +
                   from.1 as isize + to.1 as isize;
        let cost = match letter {
            b'A' => 1,
            b'B' => 10,
            b'C' => 100,
            b'D' => 1000,
            _ => panic!(),
        };
        (field, dist as i64 * cost)
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&"#".repeat(HALLWAY + 2));
        result.push_str("\n#");
        for i in 0..HALLWAY { result.push(self.0[i] as char); }
        result.push_str("#\n");
        for i in 1..=self.size() {
            let edge = if i == 1 {"#"} else {" "};
            result.push_str(&edge.repeat(EDGE));
            result.push('#');
            for j in 0..ROOMS {
                result.push(self.0[addr(j, i)] as char);
                result.push('#');
            }
            result.push_str(&edge.repeat(EDGE));
            result.push('\n');
        }
        result.push_str(&" ".repeat(EDGE));
        result.push_str(&"#".repeat(ROOMS * 2 + 1));
        result.push_str(&" ".repeat(EDGE));
        write!(f, "{}", result)
    }
}

fn find_best(field: &Field) -> Option<(i64, Vec<Move>)> {
    let mut heap: BinaryHeap<(i64, Field)> = BinaryHeap::new();
    let mut state: HashMap<Field, (i64, Vec<Move>)> = HashMap::new();
    heap.push((0, field.clone()));
    state.insert(field.clone(), (0, vec![]));

    while let Some((total, field)) = heap.pop() {
        let value = state.get(&field).unwrap().clone();
        if field.is_final() { return Some(value); }

        for move_ in field.find_moves() {
            let (next, cost) = field.apply_move(move_);
            let s1 = -total + cost;
            if let Some((s2, _)) = state.get(&next) {
                if *s2 <= s1 { continue; }
            }
            heap.push((-s1, next.clone()));

            let mut path = value.1.clone();
            path.push(move_);
            state.insert(next, (s1, path));
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input2.txt").expect("Error reading input");
    let field = Field::from(&input);
    let result =  find_best(&field).unwrap();
    println!("{} {}", result.0, result.1.len());
}
