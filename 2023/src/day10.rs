use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug)]
enum Cell {
  Horizontal,
  Vertical,
  NorthEast,
  NorthWest,
  SouthEast,
  SouthWest,
  Ground,
  Start,
}

struct Grid {
  cells: Vec<Vec<Cell>>,
  start: Point,
}

fn get_side((x1, y1): Point, (x2, y2): Point, left: bool) -> Point {
  let delta = if left {1} else {-1};
  if (x2, y2) == (x1 + 1, y1) { return (x2, y2 - delta); }
  if (x2, y2) == (x1, y1 + 1) { return (x2 + delta, y2); }
  if (x2, y2) == (x1 - 1, y1) { return (x2, y2 + delta); }
  if (x2, y2) == (x1, y1 - 1) { return (x2 - delta, y2); }
  panic!("incorrect input");
}

impl Cell {
  fn endpoints(&self, (x, y): Point) -> Option<[Point; 2]> {
    match self {
      Cell::Horizontal => Some([(x - 1, y), (x + 1, y)]),
      Cell::Vertical => Some([(x, y - 1), (x, y + 1)]),
      Cell::NorthEast => Some([(x, y - 1), (x + 1, y)]),
      Cell::NorthWest => Some([(x, y - 1), (x - 1, y)]),
      Cell::SouthEast => Some([(x, y + 1), (x + 1, y)]),
      Cell::SouthWest => Some([(x, y + 1), (x - 1, y)]),
      _ => None,
    }
  }

  fn directions() -> [Cell; 6] {
    [Cell::Horizontal, Cell::Vertical,
     Cell::NorthEast, Cell::NorthWest,
     Cell::SouthEast, Cell::SouthWest]
  }
}

impl Grid {
  fn parse(text: &str) -> Grid {
    let mut start: Option<Point> = None;
    let cells = text.lines().enumerate().map(|(y, line)| {
      line.chars().enumerate().map(|(x, symb)| {
        match symb {
          '-' => Cell::Horizontal,
          '|' => Cell::Vertical,
          'L' => Cell::NorthEast,
          'J' => Cell::NorthWest,
          'F' => Cell::SouthEast,
          '7' => Cell::SouthWest,
          '.' => Cell::Ground,
          'S' => { start = Some((x as i32, y as i32)); Cell::Start },
          _ => panic!("unknown symbol"),
        }
      }).collect()
    }).collect();
    Grid { cells, start: start.unwrap() }
  }

  fn width(&self) -> i32 { self.cells[0].len() as i32 }
  fn height(&self) -> i32 { self.cells.len() as i32 }

  fn is_valid(&self, (x, y): Point) -> bool {
    x >= 0 && x < self.width() && y >= 0 && y < self.height()
  }

  fn follow(&self, init: Cell) -> Option<Vec<Point>> {
    let mut path = vec![self.start];
    let [mut cur, last] = init.endpoints(self.start).unwrap();
    while cur != last {
      if !self.is_valid(cur) { return None; }
      let prev = *path.last().unwrap();
      path.push(cur);
      let cell = &self.cells[cur.1 as usize][cur.0 as usize];
      cur = match cell.endpoints(cur) {
        Some(adj) => if adj[0] != prev {adj[0]} else {adj[1]},
        None => return None,
      };
    }
    path.push(last);
    Some(path)
  }

  fn longest(&self) -> Vec<Point> {
    Cell::directions().into_iter()
      .filter_map(|dir| self.follow(dir))
      .map(|x| (x.len(), x))
      .max().unwrap().1
  }

  fn get_adjacent(&self, left: bool) -> HashSet<Point> {
    let mut result = HashSet::<Point>::new();
    let mut path = self.longest();
    let path_set = HashSet::<Point>::from_iter(path.iter().cloned());
    path.push(self.start);
    for p in path.windows(2) {
      let mut search = vec![
        get_side(p[0], p[1], left),
        get_side(p[1], p[0], !left)
      ];
      while let Some(cur) = search.pop() {
        if !self.is_valid(cur) || path_set.contains(&cur) ||
          result.contains(&cur) { continue; }
        let (x, y) = cur;
        for next in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
          search.push(next);
        }
        result.insert(cur);
      }
    }
    result
  }

  fn count_inner(&self) -> usize {
    let s1 = self.get_adjacent(false);
    let s2 = self.get_adjacent(true);
    [s1, s2].iter().filter(|s| s.iter().all(|&(x, y)| {
        x != 0 && x != self.width() - 1 &&
        y != 0 && y != self.height() - 1
      })).next().unwrap().len()
  }
}

pub fn run(content: &str) {
  let grid = Grid::parse(content);
  let res1 = grid.longest().len() / 2;
  let res2 = grid.count_inner();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
  const TEST_2: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
  const TEST_3: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

  #[test]
  fn small() {
    let grid = super::Grid::parse(TEST_1);
    assert_eq!(grid.longest().len(), 16);
  }

  #[test]
  fn large() {
    assert_eq!(super::Grid::parse(TEST_2).count_inner(), 8);
    assert_eq!(super::Grid::parse(TEST_3).count_inner(), 10);
  }
}
