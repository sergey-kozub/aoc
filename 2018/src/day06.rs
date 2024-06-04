use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug)]
struct Field {
  points: Vec<Point>,
  tl: Point,
  br: Point,
}

impl Field {
  fn parse(text: &str) -> Field {
    let points = text.lines().map(|s| {
      let (s1, s2) = s.split_once(", ").unwrap();
      (s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap())
    }).collect::<Vec<_>>();
    let x_min = points.iter().map(|&p| p.0).min().unwrap();
    let x_max = points.iter().map(|&p| p.0).max().unwrap();
    let y_min = points.iter().map(|&p| p.1).min().unwrap();
    let y_max = points.iter().map(|&p| p.1).max().unwrap();
    Field { points, tl: (x_min, y_min), br: (x_max, y_max) }
  }

  fn largest(&self) -> usize {
    let mut count = vec![0_usize; self.points.len()];
    let mut valid = vec![true; count.len()];
    for y in self.tl.1..=self.br.1 {
      for x in self.tl.0..=self.br.0 {
        let mut dist = self.points.iter().enumerate().map(|(k, v)| {
          ((v.0 - x).abs() + (v.1 - y).abs(), k)
        }).collect::<Vec<_>>();
        dist.sort();
        if dist[0].0 == dist[1].0 { continue; }
        let index = dist[0].1;
        count[index] += 1;
        if x == self.tl.0 || x == self.br.0 ||
           y == self.tl.1 || y == self.br.1 { valid[index] = false; }
      }
    }
    (0..count.len()).filter_map(|i| {
      if valid[i] {Some(count[i])} else {None}
    }).max().unwrap()
  }

  fn region(&self, limit: i32) -> usize {
    let mut visited = HashSet::<Point>::new();
    let mut queue = vec![
      ((self.tl.0 + self.br.0) / 2, (self.tl.1 + self.br.1) / 2)
    ];
    let mut miss = 0_usize;
    while let Some((x, y)) = queue.pop() {
      let dist = self.points.iter().map(|t| {
        (t.0 - x).abs() + (t.1 - y).abs()
      }).sum::<i32>();
      if dist >= limit { miss += 1; continue; }
      for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if visited.insert(p.clone()) { queue.push(p); }
      }
    }
    visited.len() - miss
  }
}

pub fn run(content: &str) {
  let field = Field::parse(content);
  let res1 = field.largest();
  let res2 = field.region(10000);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

  #[test]
  fn small() {
    let test = super::Field::parse(TEST);
    assert_eq!(test.largest(), 17);
  }

  #[test]
  fn large() {
    let test = super::Field::parse(TEST);
    assert_eq!(test.region(32), 16);
  }
}
