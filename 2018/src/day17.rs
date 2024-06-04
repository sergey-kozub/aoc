use std::collections::HashSet;
use std::fmt;
use std::ops::RangeInclusive;

type Point = (i32, i32);

struct Fountain {
  walls: HashSet<Point>,
  water: HashSet<Point>,
  drops: HashSet<Point>,
  x_range: RangeInclusive<i32>,
  y_range: RangeInclusive<i32>,
}

impl Fountain {
  fn parse(text: &str) -> Self {
    let num = |s: &str| s.parse::<i32>().unwrap();
    let walls = text.lines().flat_map(|line| {
      let (s1, s2) = line.split_once(", ").unwrap();
      let (lo, hi) = s2[2..].split_once("..").unwrap();
      let point = num(&s1[2..]);
      let range = num(lo)..=num(hi);
      (if s1.starts_with("x=") {
        range.map(|y| (point, y)).collect::<Vec<_>>()
      } else {
        range.map(|x| (x, point)).collect::<Vec<_>>()
      }).into_iter()
    }).collect::<HashSet<_>>();
    let iter_x = || walls.iter().map(|p| p.0);
    let iter_y = || walls.iter().map(|p| p.1);
    let x_range = iter_x().min().unwrap()-1..=iter_x().max().unwrap()+1;
    let y_range = iter_y().min().unwrap()..=iter_y().max().unwrap();
    Fountain {
      walls,
      water: HashSet::new(),
      drops: HashSet::new(),
      x_range,
      y_range,
    }
  }

  fn total(&self) -> usize {
    self.water.union(&self.drops).count()
  }

  fn drop(&mut self, (x, y): Point) {
    if y > *self.y_range.end() ||
      (y >= *self.y_range.start() && !self.drops.insert((x, y))) {
      return;
    }
    let next = (x, y + 1);
    if self.walls.contains(&next) {
      self.raise((x, y));
    } else {
      self.drop(next);
    }
  }

  fn raise(&mut self, (x, y): Point) {
    let pred = |i: &i32| -> bool {
      let (point, below) = ((*i, y), (*i, y + 1));
      !self.walls.contains(&point) &&
      (self.walls.contains(&below) || self.water.contains(&below))
    };
    let l = (*self.x_range.start()..=x).rev().take_while(pred).last().unwrap();
    let r = (x..=*self.x_range.end()).take_while(pred).last().unwrap();
    let l_bound = self.walls.contains(&(l - 1, y));
    let r_bound = self.walls.contains(&(r + 1, y));
    if l_bound && r_bound {
      for i in l..=r { self.water.insert((i, y)); }
      self.raise((x, y - 1));
      return;
    }
    for i in l..=r { self.drops.insert((i, y)); }
    if !l_bound { self.drop((l - 1, y)); }
    if !r_bound { self.drop((r + 1, y)); }
  }
}

impl fmt::Debug for Fountain {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in self.y_range.clone() {
      let line = self.x_range.clone().map(|x| {
        if self.walls.contains(&(x, y)) { return '#'; }
        if self.water.contains(&(x, y)) { return '~'; }
        if self.drops.contains(&(x, y)) { return '|'; }
        '.'
      }).collect::<String>();
      writeln!(f, "{}", line)?;
    }
    Ok(())
  }
}

pub fn run(content: &str) {
  let mut fountain = Fountain::parse(content);
  fountain.drop((500, 0));
  let res1 = fountain.total();
  let res2 = fountain.water.len();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

  #[test]
  fn small() {
    let mut test = super::Fountain::parse(TEST);
    test.drop((500, 0));
    assert_eq!(test.total(), 57);
  }
}
