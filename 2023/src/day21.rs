use std::cmp;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
  N, NE, E, SE, S, SW, W, NW
}

struct Field {
  walls: HashSet<Point>,
  start: Point,
  size: Point,
}

impl Field {
  fn parse(text: &str) -> Field {
    let width = text.lines().next().unwrap().len() as i32;
    let height = text.lines().count() as i32;
    let mut start: Option<Point> = None;
    let walls = text.lines().enumerate().flat_map(|(y, s)| {
      let mut a: Vec<Point> = vec![];
      for (x, c) in s.chars().enumerate() { match c {
        '#' => a.push((x as i32, y as i32)),
        '.' => (),
        'S' => start = Some((x as i32, y as i32)),
        _ => panic!("unknown symbol"),
      }}
      a.into_iter()
    }).collect::<HashSet<_>>();
    Field { walls, start: start.unwrap(), size: (width, height) }
  }

  fn simple(&self, count: usize) -> usize {
    let mut result = 0_usize;
    let mut prev = HashSet::<Point>::new();
    let mut cur = HashSet::from([self.start]);
    let odd = count % 2;
    let fix = |i, n| { let r = i % n; if r >= 0 {r} else {r + n} };
    for i in 0..count {
      let next = cur.iter().flat_map(|&(x, y)| {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
          .filter_map(|p| {
            if prev.contains(&p) { return None; }
            let wp = (fix(p.0, self.size.0), fix(p.1, self.size.1));
            if !self.walls.contains(&wp) {Some(p)} else {None}
          })
      }).collect::<HashSet<_>>();
      if i % 2 != odd {
        result += prev.len();
      }
      prev = cur;
      cur = next;
    }
    result + cur.len()
  }

  fn initial(&self, dir: Direction) -> Field {
    let (sx, sy) = self.start;
    let (mx, my) = (self.size.0 - 1, self.size.1 - 1);
    let start = match dir {
      Direction::N => (sx, 0),
      Direction::NE => (mx, 0),
      Direction::E => (mx, sy),
      Direction::SE => (mx, my),
      Direction::S => (sx, my),
      Direction::SW => (0, my),
      Direction::W => (0, sy),
      Direction::NW => (0, 0),
    };
    Field { walls: self.walls.clone(), start, size: self.size }
  }

  fn forward(&self, limit: i32) -> usize {
    let mut s1 = HashSet::from([self.start]);
    let mut s2 = HashSet::<Point>::new();
    let mut steps = 0_i32;
    for _ in 0..limit {
      let next = s1.iter().flat_map(|&(x, y)| {
        let mut a = Vec::<Point>::with_capacity(4);
        if x > 0 { a.push((x - 1, y)); }
        if x < self.size.0 - 1 { a.push((x + 1, y)); }
        if y > 0 { a.push((x, y - 1)); }
        if y < self.size.1 - 1 { a.push((x, y + 1)); }
        a.into_iter().filter(|p| !self.walls.contains(p) && !s2.contains(p))
      }).collect::<HashSet<_>>();
      if next.is_empty() { break; }
      s2.extend(next.into_iter());
      (s1, s2) = (s2, s1);
      steps += 1;
    }
    if steps % 2 == limit % 2 {s1.len()} else {s2.len()}
  }

  fn compute(&self, count: i32) -> usize {
    let n = self.size.0;
    assert_eq!(n % 2, 1);
    assert_eq!(n, self.size.1);
    assert_eq!(self.start, (n / 2, n / 2));

    let mut cache = HashMap::<(Direction, i32), usize>::new();
    let mut calc = |dir: Direction, steps: i32| -> usize {
      let val = cmp::min(steps, n * 2 + steps % 2);
      *cache.entry((dir, val))
        .or_insert_with(|| self.initial(dir).forward(val))
    };

    let span_x = (count + n / 2 + 1) / n;
    (1..=span_x).map(|i| {
      let rest_x = count - n * (i - 1) - n / 2 - 1;
      let v1 = [Direction::N, Direction::E, Direction::S, Direction::W]
        .into_iter().map(|d| calc(d, rest_x)).sum::<usize>();
      let span_y = (rest_x + n / 2 + 1) / n;
      let v2 = [Direction::NE, Direction::SE, Direction::SW, Direction::NW]
        .into_iter().map(|d| {
          let rest_y = rest_x - n * (span_y - 1) - n / 2 - 1;
          let repeats = |c: usize| (span_y as usize - c) / 2;
          (if span_y > 0 {calc(d, rest_y)} else {0}) +
          (if span_y > 1 {calc(d, rest_y + n)} else {0}) +
          (if span_y > 2 {calc(d, rest_y + n * 2) * repeats(1)} else {0}) +
          (if span_y > 3 {calc(d, rest_y + n * 3) * repeats(2)} else {0})
        }).sum::<usize>();
      v1 + v2
    }).sum::<usize>() + self.forward(count)
  }
}

pub fn run(content: &str) {
  let field = Field::parse(content);
  let res1 = field.simple(64);
  let res2 = field.compute(26501365);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
  const TEST_2: &str = "\
...........
......##.#.
.###..#..#.
..#.#...#..
....#.#....
.....S.....
.##......#.
.......##..
.##.#.####.
.##...#.##.
...........";

  #[test]
  pub fn small() {
    let test = super::Field::parse(TEST_1);
    assert_eq!(test.simple(6), 16);
  }

  #[test]
  pub fn large() {
    let test = super::Field::parse(TEST_1);
    assert_eq!(test.simple(100), 6536);
    let clear = super::Field::parse(TEST_2);
    assert_eq!(clear.simple(99), 7471);
    assert_eq!(clear.compute(99), 7471);
  }
}
