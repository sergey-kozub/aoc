use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::ops::Range;

type Point = (i32, i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Debug)]
struct Trench {
  pos: Point,
  dir: Direction,
  len: i32,
}

#[derive(Clone)]
struct Field {
  dig: Vec<Trench>,
}

impl Direction {
  fn delta(&self) -> (i32, i32) {
    match self {
      Direction::Up => (0, -1),
      Direction::Down => (0, 1),
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
    }
  }
}

impl Trench {
  fn parse(text: &str, pos: Point, alt: bool) -> Trench {
    let a: Vec<&str> = text.split(' ').collect();
    let (dir, len) = if alt {
      let color = i32::from_str_radix(&a[2][2..8], 16).unwrap();
      ([Direction::Right, Direction::Down, Direction::Left, Direction::Up]
       [color as usize % 16], color / 16)
    } else {
      (match a[0] {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("unknown symbol"),
      }, a[1].parse::<i32>().unwrap())
    };
    Trench { pos, dir, len }
  }

  fn end(&self) -> Point {
    let d = self.dir.delta();
    (self.pos.0 + d.0 * self.len, self.pos.1 + d.1 * self.len)
  }
}

impl Field {
  fn parse(text: &str, alt: bool) -> Field {
    let mut dig: Vec<Trench> = vec![];
    text.lines().fold((0_i32, 0_i32), |p, s| {
      let tr = Trench::parse(s, p, alt);
      dig.push(tr.clone());
      tr.end()
    });
    Field { dig }
  }

  fn simple(&self) -> i64 {
    let pos = self.dig.iter().flat_map(|t| {
      let d = t.dir.delta();
      (0..t.len).map(move |i| (t.pos.0 + d.0 * i, t.pos.1 + d.1 * i))
    }).collect::<HashSet<Point>>();
    let (rx, ry) = get_bounds(&pos);
    let init: [Point; 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    init.into_iter().filter_map(|pt| {
      let mut dig = pos.clone();
      let mut queue = vec![pt];
      while let Some((x, y)) = queue.pop() {
        if !rx.contains(&x) || !ry.contains(&y) { return None; }
        dig.insert((x, y));
        for adj in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
          if !dig.contains(&adj) { queue.push(adj); }
        }
      }
      Some(dig.len() as i64)
    }).next().unwrap()
  }

  fn fold(&self) -> i64 {
    let mut result = 0_i64;
    let mut nodes: Vec<Trench> = vec![];
    for (idx, item) in self.dig.iter().enumerate() {
      nodes.push(item.clone());
      while nodes.len() >= 4 {
        let m = nodes.len() - 4;
        let a = &nodes[m..];
        if a[1].dir != a[3].dir { break; }

        let same = a[0].dir == a[2].dir;
        let n1 = a[0].len + a[2].len * (if same {1} else {-1});
        let d1 = if n1 > 0 {a[0].dir} else {a[2].dir};
        let t1 = Trench { pos: a[0].pos, dir: d1, len: n1.abs() };
        let n2 = a[1].len + a[3].len;
        let t2 = Trench { pos: t1.end(), dir: a[1].dir, len: n2 };

        let (rx, ry) = get_box(a[1].pos, a[3].pos);
        let inside = |t: &Trench|
          rx.contains(&t.pos.0) && ry.contains(&t.pos.1);
        if self.dig[idx+1..].iter().any(inside)
          || nodes[..m].iter().any(inside) {
          break;
        }

        let (d1, d2) = (a[1].dir.delta(), a[2].dir.delta());
        let sign = (if d1.0 != 0 {d1.0 * d2.1} else {-d1.1 * d2.0}) as i64;
        let (w, h) = (a[1].len as i64, a[2].len as i64);
        result += if !same {
          sign * (w + sign) * h - (if n1 < 0 {t1.len as i64} else {0})
        } else {
          sign * w * h
        };
        nodes.truncate(m);
        nodes.extend([t1, t2]);
      }
    }
    let count = |d: Direction| nodes.iter().filter(|t| t.dir == d)
      .map(|t| t.len as i64).sum::<i64>();
    let width = count(Direction::Left) + 1;
    let height = count(Direction::Up) + 1;
    width * height + result
  }
}

impl fmt::Debug for Field {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let pos = self.dig.iter().flat_map(|t| {
      let d = t.dir.delta();
      (0..t.len).map(move |i| (t.pos.0 + d.0 * i, t.pos.1 + d.1 * i))
    }).collect::<HashSet<Point>>();
    let (rx, ry) = get_bounds(&pos);
    write!(f, "{}", ry.fold(String::new(), |s, y| {
      s + &String::from_utf8(rx.clone().map(|x| (
        if pos.contains(&(x, y)) {'#'} else {'.'}
      ) as u8).collect::<Vec<_>>()).unwrap() + "\n"
    }))
  }
}

fn get_box((x1, y1): Point, (x2, y2): Point) -> (Range<i32>, Range<i32>) {
  (cmp::min(x1, x2)..cmp::max(x1, x2) + 1,
   cmp::min(y1, y2)..cmp::max(y1, y2) + 1)
}

fn get_bounds(data: &HashSet<Point>) -> (Range<i32>, Range<i32>) {
  let x_min = data.iter().map(|p| p.0).min().unwrap();
  let x_max = data.iter().map(|p| p.0).max().unwrap();
  let y_min = data.iter().map(|p| p.1).min().unwrap();
  let y_max = data.iter().map(|p| p.1).max().unwrap();
  (x_min..x_max + 1, y_min..y_max + 1)
}

pub fn run(content: &str) {
  let res1 = Field::parse(content, false).simple();
  let res2 = Field::parse(content, true).fold();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

  #[test]
  fn small() {
    let test = super::Field::parse(TEST, false);
    assert_eq!(test.simple(), 62);
    assert_eq!(test.fold(), 62);
  }

  #[test]
  fn large() {
    let test = super::Field::parse(TEST, true);
    assert_eq!(test.fold(), 952408144115);
  }
}
