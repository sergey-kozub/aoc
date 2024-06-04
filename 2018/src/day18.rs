use std::collections::HashMap;
use std::fmt;

type Point = (usize, usize);

#[derive(Clone, Debug)]
enum Tile {
  Open,
  Trees,
  Lumberyard,
}

#[derive(Clone)]
struct Grid {
  tiles: Vec<Vec<Tile>>,
  size: Point,
}

impl Grid {
  fn parse(text: &str) -> Self {
    let tiles = text.lines().map(|line| {
      line.chars().map(|c| match c {
        '.' => Tile::Open,
        '|' => Tile::Trees,
        '#' => Tile::Lumberyard,
        _ => panic!("unknown symbol"),
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let size = (tiles[0].len(), tiles.len());
    Grid { tiles, size }
  }

  fn count(&self, (x, y): Point) -> [u8; 3] {
    let mut result = [0_u8; 3];
    for dy in 0..=2 {
      for dx in 0..=2 {
        if dy == 1 && dx == 1 { continue; }
        if (x == 0 && dx == 0) || (y == 0 && dy == 0) { continue; }
        let (nx, ny) = (x + dx - 1, y + dy - 1);
        if nx < self.size.0 && ny < self.size.1 {
          match self.tiles[ny][nx] {
            Tile::Open => result[0] += 1,
            Tile::Trees => result[1] += 1,
            Tile::Lumberyard => result[2] += 1,
          }
        }
      }
    }
    result
  }

  fn step(&self) -> Self {
    let tiles = (0..self.size.1).map(|y| {
      (0..self.size.0).map(|x| {
        let c = self.count((x, y));
        match self.tiles[y][x] {
          Tile::Open => if c[1] >= 3 {Tile::Trees} else {Tile::Open},
          Tile::Trees => if c[2] >= 3 {Tile::Lumberyard} else {Tile::Trees},
          Tile::Lumberyard =>
            if c[2] >= 1 && c[1] >= 1 {Tile::Lumberyard} else {Tile::Open},
        }
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    Grid { tiles, size: self.size }
  }

  fn score(&self) -> usize {
    let (a, b) = self.tiles.iter().map(|e| e.iter()).flatten()
      .fold((0, 0), |(u, v), t| (
        u + matches!(t, Tile::Trees) as usize,
        v + matches!(t, Tile::Lumberyard) as usize));
    a * b
  }

  fn score_after(&self, steps: usize) -> usize {
    let mut hist = HashMap::<String, usize>::new();
    let mut cur = self.clone();
    for i in 1..=steps {
      cur = cur.step();
      let key = format!("{cur:?}");
      if let Some(v) = hist.insert(key, i) {
        let rest = (steps - v) % (i - v);
        for _ in 0..rest { cur = cur.step(); }
        break;
      }
    }
    cur.score()
  }
}

impl fmt::Debug for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for line in &self.tiles {
      let s = line.iter().map(|t| match t {
        Tile::Open => '.',
        Tile::Trees => '|',
        Tile::Lumberyard => '#',
      }).collect::<String>();
      writeln!(f, "{}", s)?;
    }
    Ok(())
  }
}

pub fn run(content: &str) {
  let grid = Grid::parse(content);
  let res1 = grid.score_after(10);
  let res2 = grid.score_after(1_000_000_000);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

  #[test]
  fn small() {
    let test = super::Grid::parse(TEST);
    assert_eq!(test.score_after(10), 1147);
  }
}
