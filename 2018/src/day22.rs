use std::collections::{BinaryHeap, HashMap};

const GEO_Y0: usize = 16807;
const GEO_X0: usize = 48271;
const GEO_MOD: usize = 20183;
const SLACK: usize = 50;

#[derive(Debug)]
struct Cave {
  erosion: Vec<Vec<usize>>,
}

#[derive(Debug)]
enum Tile {
  Rocky,
  Wet,
  Narrow,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tool {
  Gear,
  Torch,
  None,
}

impl Tool {
  fn compatible(&self, tile: Tile) -> bool {
    match self {
      Tool::Gear => matches!(tile, Tile::Rocky | Tile::Wet),
      Tool::Torch => matches!(tile, Tile::Rocky | Tile::Narrow),
      Tool::None => matches!(tile, Tile::Wet | Tile::Narrow),
    }
  }
}

impl Cave {
  fn new(depth: usize, width: usize, height: usize) -> Self {
    let mut erosion: Vec<Vec<usize>> = vec![];
    erosion.push((0..=width+SLACK).map(|x| {
      (depth + if x != 0 {x * GEO_Y0} else {0}) % GEO_MOD
    }).collect());
    for y in 1..=height+SLACK {
      let first = (depth + y * GEO_X0) % GEO_MOD;
      let mut line: Vec<usize> = vec![first];
      let prev = erosion.last().unwrap();
      for x in 1..=width+SLACK {
        let (px, py) = (line[x - 1], prev[x]);
        line.push((depth + px * py) % GEO_MOD);
      }
      erosion.push(line);
    }
    erosion[height][width] = depth;
    Cave { erosion }
  }

  fn tile_at(&self, x: usize, y: usize) -> Tile {
    match self.erosion[y][x] % 3 {
      0 => Tile::Rocky,
      1 => Tile::Wet,
      2 => Tile::Narrow,
      _ => panic!(),
    }
  }

  fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if x > 0 { result.push((x - 1, y)); }
    if x < self.erosion[0].len() - 1 { result.push((x + 1, y)); }
    if y > 0 { result.push((x, y - 1)); }
    if y < self.erosion.len() - 1 { result.push((x, y + 1)); }
    result
  }

  fn find_path(&self, target_x: usize, target_y: usize) -> usize {
    type Item = (usize, usize, Tool);
    let mut heap = BinaryHeap::<(isize, Item)>::new();
    let mut visited = HashMap::<Item, isize>::new();
    heap.push((0, (0, 0, Tool::Torch)));
    while let Some((time, (x, y, tool))) = heap.pop() {
      if x == target_x && y == target_y && tool == Tool::Torch {
        return -time as usize;
      }
      if !visited.get(&(x, y, tool)).map_or(true, |t| *t < time) {
        continue;
      }
      visited.insert((x, y, tool), time);
      for (nx, ny) in self.adjacent(x, y) {
        if tool.compatible(self.tile_at(nx, ny)) {
          heap.push((time - 1, (nx, ny, tool)));
        }
      }
      for nt in [Tool::Gear, Tool::Torch, Tool::None] {
        if nt != tool && nt.compatible(self.tile_at(x, y)) {
          heap.push((time - 7, (x, y, nt)));
        }
      }
    }
    panic!("path not found");
  }

  fn score(&self, width: usize, height: usize) -> usize {
    self.erosion.iter().take(height + 1).flat_map(|a| {
      a.iter().take(width + 1).map(|v| v % 3)
    }).sum::<usize>()
  }
}

pub fn run(content: &str) {
  let lines = content.lines().collect::<Vec<_>>();
  let parse = |s: &str| s.parse::<usize>().unwrap();
  let depth = parse(lines[0].split_once("depth: ").unwrap().1);
  let target = lines[1].split_once("target: ").unwrap().1
    .split(',').map(parse).collect::<Vec<_>>();

  let [width, height]: [usize; 2] = target.try_into().unwrap();
  let cave = Cave::new(depth, width, height);
  let res1 = cave.score(width, height);
  let res2 = cave.find_path(width, height);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn small() {
    let cave = super::Cave::new(510, 10, 10);
    assert_eq!(cave.score(10, 10), 114);
  }

  #[test]
  fn large() {
    let cave = super::Cave::new(510, 10, 10);
    assert_eq!(cave.find_path(10, 10), 45);
  }
}
