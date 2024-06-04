use regex::Regex;
use std::cmp;
use std::collections::HashSet;

type Point = (i64, i64);

#[derive(Debug)]
struct Star {
  position: Point,
  velocity: Point,
}

#[derive(Debug)]
struct Sky {
  stars: Vec<Star>,
}

impl Star {
  fn at(&self, time: i64) -> Point {
    (self.position.0 + time * self.velocity.0,
     self.position.1 + time * self.velocity.1)
  }
}

impl Sky {
  fn parse(text: &str) -> Sky {
    let re = Regex::new(&format!(
      "position=<{0},{0}> velocity=<{0},{0}>", r"\s*(-?\d+)"
    )).unwrap();
    let stars = re.captures_iter(text).map(|c| {
      let a = c.extract::<4>().1.into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
      Star { position: (a[0], a[1]), velocity: (a[2], a[3]) }
    }).collect::<Vec<_>>();
    Sky { stars }
  }

  fn get_box(&self, time: i64) -> (Point, Point) {
    let init = ((i64::MAX, i64::MAX), (i64::MIN, i64::MIN));
    self.stars.iter().fold(init, |(l, r), star| {
      let (x, y) = star.at(time);
      ((cmp::min(l.0, x), cmp::min(l.1, y)),
       (cmp::max(r.0, x), cmp::max(r.1, y)))
    })
  }

  fn find_time(&self) -> i64 {
    let size = |(l, r): (Point, Point)| (r.0 - l.0) * (r.1 - l.1);
    let mut min_time = 0_i64;
    let mut min_size = size(self.get_box(0));
    for time in 1..=1_000_000 {
      let cur = size(self.get_box(time));
      if cur < min_size { (min_time, min_size) = (time, cur); }
      if cur > min_size * 2 { break; }
    }
    min_time
  }

  fn get_output(&self) -> String {
    let time = self.find_time();
    let set = self.stars.iter().map(|t| t.at(time)).collect::<HashSet<_>>();
    let ((x1, y1), (x2, y2)) = self.get_box(time);
    (y1..=y2).fold(String::new(), |s, y| s + &(x1..=x2).map(|x| {
      if set.contains(&(x, y)) {'#'} else {'.'}
    }).collect::<String>() + "\n")
  }
}

pub fn run(content: &str) {
  let sky = Sky::parse(content);
  println!("{}{}", sky.get_output(), sky.find_time());
}
