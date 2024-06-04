use std::collections::HashSet;
use std::fmt;
use std::ops::RangeInclusive;

type Point = (u32, u32, u32);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Brick {
  cubes: Vec<Point>,
  vertical: bool,
  ordinal: usize,
}

#[derive(Clone)]
struct Field {
  bricks: Vec<Brick>,
  filled: HashSet<Point>,
}

impl Brick {
  fn parse(text: &str, ordinal: usize) -> Brick {
    let (s1, s2) = text.split_once('~').unwrap();
    let get = |s: &str| -> Point {
      let a = s.split(',').map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
      (a[0], a[1], a[2])
    };
    let (a, b) = (get(s1), get(s2));
    let cubes = (a.0..=b.0).flat_map(|x|
      (a.1..=b.1).flat_map(move |y|
        (a.2..=b.2).map(move |z| (x, y, z))))
        .collect::<Vec<_>>();
    Brick { cubes, vertical: a.2 != b.2, ordinal }
  }

  fn fall(&self, filled: &HashSet<Point>) -> Option<Brick> {
    let sub = |p: Point, n: u32| -> Point { (p.0, p.1, p.2 - n) };
    (1..self.cubes[0].2).take_while(|i| {
      if !self.vertical {
        self.cubes.iter().all(|&p| !filled.contains(&sub(p, *i)))
      } else {
        !filled.contains(&sub(self.cubes[0], *i))
      }
    }).max().map(|n| {
      Brick {
        cubes: self.cubes.iter().map(|&x| sub(x, n)).collect::<Vec<_>>(),
        vertical: self.vertical,
        ordinal: self.ordinal,
      }
    })
  }
}

impl Field {
  fn parse(text: &str) -> Field {
    let bricks = text.lines().enumerate().map(|(k, v)| Brick::parse(v, k))
      .collect::<Vec<_>>();
    let filled = bricks.iter().flat_map(|x| x.cubes.iter().copied())
      .collect::<HashSet<_>>();
    Field { bricks, filled }
  }

  fn add(&mut self, brick: Brick) {
    self.filled.extend(brick.cubes.iter().copied());
    self.bricks.push(brick);
  }

  fn remove(&mut self, index: usize) -> Brick {
    let brick = self.bricks.swap_remove(index);
    brick.cubes.iter().for_each(|p| { self.filled.remove(p); });
    brick
  }

  fn fall(&mut self, index: usize) -> bool {
    let brick = self.remove(index);
    match brick.fall(&self.filled) {
      Some(new) => { self.add(new); true },
      None => { self.add(brick); false }
    }
  }

  fn settle(&mut self) -> bool {
    let mut changed = false;
    while {
      (0..self.bricks.len()).rev()
        .filter(|i| self.fall(*i)).count() > 0
    } { changed = true; }
    changed
  }

  fn count_safe(&self) -> usize {
    (0..self.bricks.len()).filter(|i| {
      let mut copy = self.clone();
      copy.remove(*i);
      !copy.settle()
    }).count()
  }

  fn count_fall(&self, index: usize) -> usize {
    let mut copy = self.clone();
    copy.remove(index);
    copy.settle();
    copy.bricks.iter().filter(|&a| {
      let idx = self.bricks.iter()
        .position(|b| a.ordinal == b.ordinal).unwrap();
      self.bricks[idx] != *a
    }).count()
  }

  fn range_x(&self) -> RangeInclusive<u32> {
    let iter_x = || self.filled.iter().map(|p| p.0);
    iter_x().min().unwrap()..=iter_x().max().unwrap()
  }
  fn range_y(&self) -> RangeInclusive<u32> {
    let iter_y = || self.filled.iter().map(|p| p.1);
    iter_y().min().unwrap()..=iter_y().max().unwrap()
  }
  fn range_z(&self) -> RangeInclusive<u32> {
    let iter_z = || self.filled.iter().map(|p| p.2);
    iter_z().min().unwrap()..=iter_z().max().unwrap()
  }

  fn view_x(&self) -> String {
    (1..=*self.range_z().end()).rev().fold(String::new(), |s, z| {
      s + &String::from_iter(self.range_x().map(|x| {
        as_char(self.bricks.iter().filter_map(|v| {
          if v.cubes.iter().any(|p| p.0 == x && p.2 == z)
            {Some(v.ordinal)} else {None}
        }).collect::<Vec<_>>())
      })) + "\n"
    })
  }
  fn view_y(&self) -> String {
    (1..=*self.range_z().end()).rev().fold(String::new(), |s, z| {
      s + &String::from_iter(self.range_y().map(|y| {
        as_char(self.bricks.iter().filter_map(|v| {
          if v.cubes.iter().any(|p| p.1 == y && p.2 == z)
            {Some(v.ordinal)} else {None}
        }).collect::<Vec<_>>())
      })) + "\n"
    })
  }
}

fn as_char(a: Vec<usize>) -> char {
  if a.len() == 1 {(65 + a[0] as u8) as char} else
  if a.len() > 1 {(48 + a.len() as u8) as char} else {'.'}
}

impl fmt::Debug for Field {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[X]\n{}\n[Y]\n{}", self.view_x(), self.view_y())
  }
}

pub fn run(content: &str) {
  let mut field = Field::parse(content);
  field.settle();
  let res1 = field.count_safe();
  let res2 = (0..field.bricks.len())
    .map(|i| field.count_fall(i)).sum::<usize>();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

  #[test]
  fn small() {
    let mut test = super::Field::parse(TEST);
    test.settle();
    assert_eq!(test.count_safe(), 5);
  }

  #[test]
  fn large() {
    let mut test = super::Field::parse(TEST);
    test.settle();
    let fall = (0..7).map(|i| test.count_fall(i))
      .filter(|&n| n > 0).collect::<Vec<_>>();
    assert_eq!(fall, [1, 6]);
  }
}
