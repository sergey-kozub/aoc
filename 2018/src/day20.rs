use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

type Coord = (i32, i32);

#[derive(Clone, Copy, Debug)]
enum Direction {
  North,
  South,
  East,
  West,
}

#[derive(Debug)]
enum Pattern {
  Move(Direction),
  Sequence(Vec<Pattern>),
  Branch(Vec<Pattern>),
}

#[derive(Debug)]
enum Tile {
  Unknown,
  Door,
}

#[derive(Debug)]
struct Room {
  north: Tile,
  west: Tile,
}

#[derive(Debug)]
struct Maze {
  rooms: HashMap<Coord, Room>,
  fixed: bool,
}

impl Direction {
  fn from(name: char) -> Direction {
    match name {
      'N' => Direction::North,
      'S' => Direction::South,
      'E' => Direction::East,
      'W' => Direction::West,
      _ => panic!(),
    }
  }

  fn next(&self, (x, y): Coord) -> Coord {
    match self {
      Direction::North => (x, y - 1),
      Direction::South => (x, y + 1),
      Direction::East => (x + 1, y),
      Direction::West => (x - 1, y),
    }
  }
}

impl Pattern {
  fn parse(text: &str) -> Pattern {
    let mut acc: Vec<Pattern> = vec![];
    let mut size: Vec<usize> = vec![];
    for symb in text.chars() {
      match symb {
        '^' => { assert!(size.is_empty()); size.push(0); }
        '$' => { assert!(size.len() == 1); break; }
        'N'|'S'|'E'|'W' => {
          acc.push(Pattern::Move(Direction::from(symb)));
          *size.last_mut().unwrap() += 1;
        },
        '(' => size.extend([0, 0]),
        '|'|')' => {
          for i in 0..(if symb == '|' {1} else {2}) {
            let a = acc.split_off(acc.len() - size.pop().unwrap());
            let p = if i == 0 {Pattern::Sequence(a)} else {Pattern::Branch(a)};
            acc.push(p);
            *size.last_mut().unwrap() += 1;
          }
          if symb == '|' { size.push(0); }
        },
        _ => panic!("unknown symbol"),
      }
    }
    Pattern::Sequence(acc)
  }

  fn to_string(&self) -> String {
    match self {
      Pattern::Move(Direction::North) => String::from("N"),
      Pattern::Move(Direction::South) => String::from("S"),
      Pattern::Move(Direction::East) => String::from("E"),
      Pattern::Move(Direction::West) => String::from("W"),
      Pattern::Sequence(seq) =>
        seq.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""),
      Pattern::Branch(seq) => {
        let s = seq.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("|");
        String::from("(") + &s + ")"
      }
    }
  }
}

impl Room {
  fn h_door(&self) -> bool { matches!(self.west, Tile::Door) }
  fn v_door(&self) -> bool { matches!(self.north, Tile::Door) }
}

impl Maze {
  fn new() -> Maze {
    let mut maze = Maze { rooms: HashMap::new(), fixed: false };
    maze.get_room((0, 0));
    maze
  }

  fn has_door(&self, (x, y): Coord, dir: Direction) -> bool {
    match dir {
      Direction::North =>
        self.rooms.get(&(x, y)).map_or(false, |r| r.v_door()),
      Direction::South =>
        self.rooms.get(&(x, y + 1)).map_or(false, |r| r.v_door()),
      Direction::East =>
        self.rooms.get(&(x + 1, y)).map_or(false, |r| r.h_door()),
      Direction::West =>
        self.rooms.get(&(x, y)).map_or(false, |r| r.h_door()),
    }
  }

  fn traverse(&self, count: usize) -> usize {
    let mut queue = VecDeque::<(Coord, usize)>::from([((0, 0), 0)]);
    let mut visited = HashSet::<Coord>::from([queue[0].0]);
    let mut result = 0;
    while let Some((pos, len)) = queue.pop_front() {
      let (x, y) = pos;
      let mut moves: Vec<Coord> = vec![];
      if self.has_door(pos, Direction::North) { moves.push((x, y - 1)); }
      if self.has_door(pos, Direction::South) { moves.push((x, y + 1)); }
      if self.has_door(pos, Direction::East) { moves.push((x + 1, y)); }
      if self.has_door(pos, Direction::West) { moves.push((x - 1, y)); }
      for new in moves {
        if !visited.insert(new) { continue; }
        queue.push_back((new, len + 1));
      }
      if count == 0 { result = len; }
      else if len >= count { result += 1; }
    }
    result
  }

  fn get_room(&mut self, pos: Coord) -> &mut Room {
    self.rooms.entry(pos).or_insert_with(||
      Room { north: Tile::Unknown, west: Tile::Unknown })
  }

  fn step(&mut self, pos: Coord, dir: Direction) -> Coord {
    let new = dir.next(pos);
    match dir {
      Direction::North => self.get_room(pos).north = Tile::Door,
      Direction::South => self.get_room(new).north = Tile::Door,
      Direction::East => self.get_room(new).west = Tile::Door,
      Direction::West => self.get_room(pos).west = Tile::Door,
    };
    new
  }

  fn step_all(&mut self, start: Vec<Coord>, pattern: &Pattern) -> Vec<Coord> {
    match pattern {
      Pattern::Move(dir) => {
        start.into_iter().map(|pos| self.step(pos, *dir)).collect::<Vec<_>>()
      },
      Pattern::Sequence(seq) => {
        seq.iter().fold(start, |pos, sub| self.step_all(pos, sub))
      },
      Pattern::Branch(seq) => {
        let set: HashSet<Coord> = HashSet::from_iter(seq.iter().flat_map(|sub| {
          self.step_all(start.clone(), sub).into_iter()
        }));
        Vec::from_iter(set.into_iter())
      },
    }
  }

  #[allow(dead_code)]
  fn finalize(&mut self) {
    let new = self.rooms.iter().flat_map(|(&(x, y), room)| {
      [if room.v_door() {Some((x, y - 1))} else {None},
       if room.h_door() {Some((x - 1, y))} else {None}
      ].into_iter().filter_map(|t| t)
    }).collect::<Vec<_>>();
    new.into_iter().for_each(|pos| { self.get_room(pos); });
    self.fixed = true;
  }
}

impl fmt::Display for Pattern {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

impl fmt::Display for Maze {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let x_iter = || self.rooms.keys().map(|(x, _)| *x);
    let y_iter = || self.rooms.keys().map(|(_, y)| *y);
    let (x_min, x_max) = (x_iter().min().unwrap(), x_iter().max().unwrap() + 1);
    let (y_min, y_max) = (y_iter().min().unwrap(), y_iter().max().unwrap() + 1);
    let wall = if self.fixed {'#'} else {'?'};
    for y in y_min..=y_max {
      let items = (x_min..=x_max).map(|x| {
        match self.rooms.get(&(x, y)) {
          Some(room) => {
            let c1 = if room.v_door() {'-'} else {wall};
            let c2 = if room.h_door() {'|'} else {wall};
            ['#', c1, c2, if x == 0 && y == 0 {'X'} else {'.'}]
          },
          None => {
            let h = self.rooms.contains_key(&(x - 1, y));
            let v = self.rooms.contains_key(&(x, y - 1));
            let last = x == x_max && y == y_max &&
              self.rooms.contains_key(&(x - 1, y - 1));
            [if h || v || last {'#'} else {' '},
             if v {wall} else {' '},
             if h {wall} else {' '}, ' ']
          },
        }
      }).collect::<Vec<_>>();
      let s1 = items.iter().flat_map(|a| [a[0], a[1]]).collect::<String>();
      let s2 = items.iter().flat_map(|a| [a[2], a[3]]).collect::<String>();
      writeln!(f, "{}", s1)?;
      writeln!(f, "{}", s2)?;
    }
    Ok(())
  }
}

pub fn run(content: &str) {
  let pattern = Pattern::parse(content);
  let mut maze = Maze::new();
  maze.step_all(vec![(0, 0)], &pattern);
  let res1 = maze.traverse(0);
  let res2 = maze.traverse(1000);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "^ENWWW(NEEE|SSE(EE|N))$";
  const TEST_2: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
  const TEST_3: &str = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
  const TEST_4: &str =
    "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

  #[test]
  fn small() {
    let longest_path = |s: &str| -> usize {
      let pattern = super::Pattern::parse(s);
      let mut maze = super::Maze::new();
      maze.step_all(vec![(0, 0)], &pattern);
      maze.traverse(0)
    };
    assert_eq!(longest_path(TEST_1), 10);
    assert_eq!(longest_path(TEST_2), 18);
    assert_eq!(longest_path(TEST_3), 23);
    assert_eq!(longest_path(TEST_4), 31);
  }
}
