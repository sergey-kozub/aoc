use std::collections::{HashMap, VecDeque};
use std::slice::Iter;

type Point = (u32, u32);

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct HeatState {
  pos: Point,
  dir: Direction,
  step: u32,
}

struct HeatMap {
  data: Vec<Vec<u8>>,
}

impl Direction {
  fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 4] = [
      Direction::Up, Direction::Down, Direction::Left, Direction::Right];
    DIRECTIONS.iter()
  }

  fn opposite(&self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

impl HeatMap {
  fn parse(text: &str) -> HeatMap {
    let data = text.lines().map(|s| {
      s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect();
    HeatMap { data }
  }

  fn width(&self) -> u32 { self.data[0].len() as u32 }
  fn height(&self) -> u32 { self.data.len() as u32 }

  fn step(&self, (x, y): Point, dir: Direction) -> Option<Point> {
    match dir {
      Direction::Up => if y != 0 {Some((x, y - 1))} else {None},
      Direction::Down => if y < self.height()-1 {Some((x, y + 1))} else {None},
      Direction::Left => if x != 0 {Some((x - 1, y))} else {None},
      Direction::Right => if x < self.width()-1 {Some((x + 1, y))} else {None},
    }
  }

  fn travel(&self, max_steps: u32, min_turn: u32) -> u32 {
    let init = HeatState {pos: (0, 0), dir: Direction::Right, step: 0};
    let mut queue = VecDeque::from([(init, 0_u32)]);
    let mut best = HashMap::<HeatState, u32>::new();
    let dest = (self.width() - 1, self.height() - 1);
    while let Some((cur, loss)) = queue.pop_front() {
      for &dir in Direction::iterator() {
        if dir == cur.dir.opposite() { continue; }
        let step = if dir == cur.dir {cur.step + 1} else {1};
        if step > max_steps { continue; }
        if cur.step < min_turn && dir != cur.dir && loss != 0 { continue; }
        if let Some(new_pos) = self.step(cur.pos, dir) {
          if new_pos == dest && step < min_turn { continue; }
          let state = HeatState { pos: new_pos, dir, step };
          let delta = self.data[new_pos.1 as usize][new_pos.0 as usize] as u32;
          let new_loss = loss + delta;
          if !best.get(&state).map(|v| *v <= new_loss).unwrap_or(false) {
            best.insert(state.clone(), new_loss);
            queue.push_back((state, new_loss));
          }
        }
      }
    }
    best.iter().filter_map(|(k, v)| {
      if k.pos == dest {Some(*v)} else {None}
    }).min().unwrap()
  }
}

pub fn run(content: &str) {
  let hmap = HeatMap::parse(content);
  let res1 = hmap.travel(3, 0);
  let res2 = hmap.travel(10, 4);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

  #[test]
  fn small() {
    let test = super::HeatMap::parse(TEST);
    assert_eq!(test.travel(3, 0), 102);
  }

  #[test]
  fn large() {
    let test = super::HeatMap::parse(TEST);
    assert_eq!(test.travel(10, 4), 94);
  }
}
