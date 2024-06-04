use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug)]
enum Cell {
  Floor,
  Wall,
  SlopeDown,
  SlopeRight,
}

#[derive(Debug)]
struct Room {
  entry: Vec<Point>,
  exit: Vec<Point>,
  floor: HashSet<Point>,
  target: Vec<usize>,
}

struct Maze {
  rooms: Vec<Room>,
}

impl Room {
  fn new(cells: &[Vec<Cell>], start: Point) -> Room {
    let mut entry: Vec<Point> = vec![];
    let mut exit: Vec<Point> = vec![];
    let mut floor = HashSet::<Point>::new();
    let mut queue = vec![start];
    let mut update = |p: Point, inward: bool| {
      if inward { entry.push(p); } else { exit.push(p); }
    };
    while let Some((x, y)) = queue.pop() {
      floor.insert((x, y));
      for pt in [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)] {
        let (nx, ny) = pt;
        if ny < 0 || ny >= cells.len() as i32 { continue; }
        match cells[ny as usize][nx as usize] {
          Cell::Floor => if !floor.contains(&pt) { queue.push(pt); },
          Cell::Wall => (),
          Cell::SlopeDown => update(pt, floor.contains(&(nx, ny + 1))),
          Cell::SlopeRight => update(pt, floor.contains(&(nx + 1, ny))),
        };
      }
    }
    Room { entry, exit, floor, target: vec![] }
  }
}

impl Maze {
  fn parse(text: &str) -> Maze {
    let cells = text.lines().map(|line| {
      line.chars().map(|c| match c {
        '.' => Cell::Floor,
        '#' => Cell::Wall,
        'v' => Cell::SlopeDown,
        '>' => Cell::SlopeRight,
        _ => panic!("unknown symbol"),
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut visited = HashSet::<Point>::new();
    let mut rooms = Vec::<Room>::new();
    for (y, a) in cells.iter().enumerate() {
      for (x, c) in a.iter().enumerate() {
        let p = (x as i32, y as i32);
        if matches!(c, Cell::Floor) && !visited.contains(&p) {
          let room = Room::new(&cells, p);
          visited.extend(room.floor.iter().copied());
          rooms.push(room);
        }
      }
    }

    for idx in 0..rooms.len() {
      rooms[idx].target = rooms[idx].exit.iter().map(|p| {
        rooms.iter().position(|r| r.entry.contains(&p)).unwrap()
      }).collect::<Vec<_>>();
    }
    Maze { rooms }
  }

  fn make_dry(&mut self) {
    for idx in 0..self.rooms.len() {
      let a = self.rooms[idx].entry.iter().map(|p| {
        self.rooms.iter().position(|r| r.exit.contains(&p)).unwrap()
      }).collect::<Vec<_>>();
      self.rooms[idx].target.extend(a.into_iter());
    }
  }

  fn descend(&self, index: usize, visited: &HashSet<usize>) -> usize {
    let room = &self.rooms[index];
    let mut cur = visited.clone();
    cur.insert(index);
    if room.exit.is_empty() {
      cur.iter().map(|&i| self.rooms[i].floor.len() + 1).sum::<usize>()
    } else {
      room.target.iter().filter_map(|i| {
        if !cur.contains(i) {Some(self.descend(*i, &cur))} else {None}
      }).max().unwrap_or(0)
    }
  }

  fn max_path(&self) -> usize {
    self.descend(0, &HashSet::new()) - 2
  }
}

pub fn run(content: &str) {
  let mut maze = Maze::parse(content);
  let res1 = maze.max_path();
  maze.make_dry();
  let res2 = maze.max_path();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

  #[test]
  fn small() {
    let test = super::Maze::parse(TEST);
    assert_eq!(test.max_path(), 94);
  }

  #[test]
  fn large() {
    let mut test = super::Maze::parse(TEST);
    test.make_dry();
    assert_eq!(test.max_path(), 154);
  }
}
