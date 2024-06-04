use regex::Regex;
use std::cmp;
use std::ops::RangeInclusive;

type Coord = (i64, i64, i64);
type Side = RangeInclusive<i64>;

#[derive(Debug)]
struct Nanobot {
  position: Coord,
  range: i64,
}

#[derive(Debug)]
struct Cube {
  x: Side,
  y: Side,
  z: Side,
}

impl Nanobot {
  fn parse_all(text: &str) -> Vec<Self> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    text.lines().map(|line| {
      let a: Vec<i64> = re.captures(line).unwrap().iter().skip(1)
        .map(|m| m.unwrap().as_str().parse::<i64>().unwrap()).collect();
      Nanobot { position: (a[0], a[1], a[2]), range: a[3] }
    }).collect()
  }

  fn distance(&self, other: &Nanobot) -> i64 {
    (self.position.0 - other.position.0).abs() +
    (self.position.1 - other.position.1).abs() +
    (self.position.2 - other.position.2).abs()
  }
}

impl Cube {
  fn new(x: &Side, y: &Side, z: &Side) -> Self {
    Cube { x: x.clone(), y: y.clone(), z: z.clone() }
  }

  fn x_size(&self) -> i64 { *self.x.end() - *self.x.start() + 1 }
  fn y_size(&self) -> i64 { *self.y.end() - *self.y.start() + 1 }
  fn z_size(&self) -> i64 { *self.z.end() - *self.z.start() + 1 }

  fn is_unit(&self) -> bool {
    self.x_size() == 1 && self.y_size() == 1 && self.z_size() == 1
  }

  fn contains(&self, (x, y, z): Coord) -> bool {
    self.x.contains(&x) && self.y.contains(&y) && self.z.contains(&z)
  }

  fn split(&self) -> Option<[Cube; 2]> {
    let max_size = [self.x_size(), self.y_size(), self.z_size()]
      .into_iter().max().unwrap();
    if max_size == 1 { return None; }
    let split_side = |side: &Side| {
      let (l, r) = (*side.start(), *side.end());
      let m = (l + r) / 2;
      (l..=m, m+1..=r)
    };
    if self.x_size() == max_size {
      let (l, r) = split_side(&self.x);
      Some([Cube::new(&l, &self.y, &self.z), Cube::new(&r, &self.y, &self.z)])
    } else if self.y_size() == max_size {
      let (l, r) = split_side(&self.y);
      Some([Cube::new(&self.x, &l, &self.z), Cube::new(&self.x, &r, &self.z)])
    } else {
      let (l, r) = split_side(&self.z);
      Some([Cube::new(&self.x, &self.y, &l), Cube::new(&self.x, &self.y, &r)])
    }
  }

  fn intersects(&self, bot: &Nanobot) -> bool {
    let dist = |s: &Side, t: i64| {
      if t < *s.start() {*s.start() - t} else
      if t > *s.end() {t - *s.end()} else {0}
    };
    let check = |w: &Side, h: &Side, u: i64, (x, y, z): Coord| {
      (u - z).abs() <= bot.range - dist(w, x) - dist(h, y)
    };
    let check2 = |w: &Side, h: &Side, v: &Side, p: Coord| {
      check(w, h, *v.start(), p) || check(w, h, *v.end(), p)
    };
    let (x, y, z) = bot.position;
    self.contains(bot.position)
      || check2(&self.x, &self.y, &self.z, (x, y, z))
      || check2(&self.x, &self.z, &self.y, (x, z, y))
      || check2(&self.y, &self.z, &self.x, (y, z, x))
  }
}

fn in_range_largest(bots: &[Nanobot]) -> usize {
  let max_range = bots.iter().map(|b| b.range).max().unwrap();
  let bot = bots.iter().filter(|b| b.range == max_range).next().unwrap();
  bots.iter().filter(|b| bot.distance(b) <= bot.range).count()
}

fn search_max_coverage(bots: &[Nanobot]) -> i64 {
  let x_iter = || bots.iter().map(|b| b.position.0);
  let y_iter = || bots.iter().map(|b| b.position.1);
  let z_iter = || bots.iter().map(|b| b.position.2);
  let x = x_iter().min().unwrap()..=x_iter().max().unwrap();
  let y = y_iter().min().unwrap()..=y_iter().max().unwrap();
  let z = y_iter().min().unwrap()..=z_iter().max().unwrap();

  let run_pass = |limit: i64| {
    let mut queue = Vec::<Cube>::from([Cube::new(&x, &y, &z)]);
    let mut best = i64::MIN;
    while let Some(cube) = queue.pop() {
      let count = bots.iter().filter(|&b| cube.intersects(b)).count() as i64;
      if count <= (if limit < 0 {best} else {limit}) { continue; }
      if cube.is_unit() {
        let d = || cube.x.end().abs() + cube.y.end().abs() + cube.z.end().abs();
        best = cmp::max(best, if limit < 0 {count} else {-d()});
      } else {
        queue.extend(cube.split().unwrap());
      }
    }
    best
  };
  let max_count = run_pass(-1);
  -run_pass(max_count - 1)
}

pub fn run(content: &str) {
  let bots = Nanobot::parse_all(content);
  let res1 = in_range_largest(&bots);
  let res2 = search_max_coverage(&bots);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
  const TEST_2: &str = "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

  #[test]
  fn small() {
    let bots = super::Nanobot::parse_all(TEST_1);
    assert_eq!(super::in_range_largest(&bots), 7);
  }

  #[test]
  fn large() {
    let bots = super::Nanobot::parse_all(TEST_2);
    assert_eq!(super::search_max_coverage(&bots), 36);
  }
}
