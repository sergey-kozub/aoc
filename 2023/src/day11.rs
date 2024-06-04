use std::cmp;
use std::collections::HashSet;

type Point = (u32, u32);

#[derive(Debug)]
struct StarMap {
  galaxies: Vec<Point>,
  horizontal: HashSet<u32>,
  vertical: HashSet<u32>,
}

impl StarMap {
  fn parse(text: &str) -> StarMap {
    let height = text.lines().count() as u32;
    let width = text.lines().next().unwrap().len() as u32;
    let galaxies = text.lines().enumerate().flat_map(|(y, s)| {
      s.chars().enumerate().filter_map(move |(x, c)| {
        if c == '#' {Some((x as u32, y as u32))} else {None}
      })
    }).collect::<Vec<Point>>();
    let mut horizontal = HashSet::from_iter(0..height);
    let mut vertical = HashSet::from_iter(0..width);
    for (x, y) in &galaxies {
      horizontal.remove(y);
      vertical.remove(x);
    }
    StarMap { galaxies, horizontal, vertical }
  }

  fn distance(&self, a: Point, b: Point, exp: u64) -> u64 {
    let (x1, x2) = (cmp::min(a.0, b.0), cmp::max(a.0, b.0));
    let (y1, y2) = (cmp::min(a.1, b.1), cmp::max(a.1, b.1));
    let xs = (x1..x2).filter(|x| self.vertical.contains(&x)).count() as u32;
    let ys = (y1..y2).filter(|y| self.horizontal.contains(&y)).count() as u32;
    (x2 - x1) as u64 + (y2 - y1) as u64 + (xs + ys) as u64 * (exp - 1)
  }

  fn sum_pairs(&self, exp: u64) -> u64 {
    self.galaxies.iter().flat_map(|&a| {
      self.galaxies.iter().filter(move |&&b| a < b)
        .map(move |&b| self.distance(a, b, exp))
    }).sum()
  }
}

pub fn run(content: &str) {
  let data = StarMap::parse(content);
  println!("{} {}", data.sum_pairs(2), data.sum_pairs(1_000_000));
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

  #[test]
  fn small() {
    let test = super::StarMap::parse(TEST);
    assert_eq!(test.sum_pairs(2), 374);
  }

  #[test]
  fn large() {
    let test = super::StarMap::parse(TEST);
    assert_eq!(test.sum_pairs(10), 1030);
    assert_eq!(test.sum_pairs(100), 8410);
  }
}
