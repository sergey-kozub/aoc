
type Point = (i32, i32, i32, i32);
const MERGE_DISTANCE: i32 = 3;

fn parse_all(text: &str) -> Vec<Point> {
  text.lines().map(|s| {
    let a = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    (a[0], a[1], a[2], a[3])
  }).collect()
}

fn distance(a: &Point, b: &Point) -> i32 {
  (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

#[derive(Debug)]
struct Cluster {
  points: Vec<Point>,
}

impl Cluster {
  fn new(init: Point) -> Self {
    Cluster { points: vec![init] }
  }

  fn merge(&mut self, other: Cluster) {
    self.points.extend(other.points.into_iter());
  }

  fn distance(&self, p: &Point) -> i32 {
    self.points.iter().map(|x| distance(x, p)).min().unwrap()
  }

  fn can_merge(&self, other: &Cluster) -> bool {
    self.points.iter().any(|x| other.distance(x) <= MERGE_DISTANCE)
  }
}

fn merge_all(points: &[Point]) -> Vec<Cluster> {
  let mut result: Vec<Cluster> = vec![];
  for p in points {
    let mut it = result.iter_mut().filter(|c| c.distance(p) <= MERGE_DISTANCE);
    match it.next() {
      Some(c) => c.points.push(*p),
      None => result.push(Cluster::new(*p)),
    }
  }
  for i in (0..result.len()).rev() {
    let mut it = (0..i).filter(|&j| result[j].can_merge(&result[i]));
    if let Some(j) = it.next() {
      let c = result.swap_remove(i);
      result[j].merge(c);
    }
  }
  result
}

pub fn run(content: &str) {
  let points = parse_all(content);
  let clusters = merge_all(&points);
  println!("{}", clusters.len());
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";
  const TEST_2: &str = "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
  const TEST_3: &str = "\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
  const TEST_4: &str = "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";

  #[test]
  fn small() {
    let verify = |s: &str| super::merge_all(&super::parse_all(s)).len();
    assert_eq!(verify(TEST_1), 2);
    assert_eq!(verify(TEST_2), 4);
    assert_eq!(verify(TEST_3), 3);
    assert_eq!(verify(TEST_4), 8);
  }
}
