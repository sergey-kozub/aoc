use std::collections::HashSet;

type Point = (u32, u32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Copy, Debug)]
enum Cell {
  Empty,
  MirrorL,
  MirrorR,
  SplitterH,
  SplitterV,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Beam {
  pos: Point,
  dir: Direction,
}

struct Field {
  cells: Vec<Vec<Cell>>,
}

impl Direction {
  fn vertical(&self) -> bool {
    matches!(self, Direction::Up | Direction::Down)
  }
  fn horizontal(&self) -> bool {
    matches!(self, Direction::Left | Direction::Right)
  }
}

impl Cell {
  fn redir(&self, dir: Direction) -> Vec<Direction> {
    match self {
      Cell::Empty => vec![dir],
      Cell::MirrorL => vec![match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
      }],
      Cell::MirrorR => vec![match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
      }],
      Cell::SplitterH if dir.horizontal() => vec![dir],
      Cell::SplitterH => vec![Direction::Left, Direction::Right],
      Cell::SplitterV if dir.vertical() => vec![dir],
      Cell::SplitterV => vec![Direction::Up, Direction::Down],
    }
  }
}

impl Field {
  fn parse(text: &str) -> Field {
    let cells = text.lines().map(|s| {
      s.chars().map(|c| match c {
        '.' => Cell::Empty,
        '/' => Cell::MirrorL,
        '\\' => Cell::MirrorR,
        '-' => Cell::SplitterH,
        '|' => Cell::SplitterV,
        _ => panic!("unknown symbol"),
      }).collect()
    }).collect();
    Field { cells }
  }

  fn width(&self) -> u32 { self.cells[0].len() as u32 }
  fn height(&self) -> u32 { self.cells.len() as u32 }

  fn advance(&self, dir: Direction, (x, y): Point) -> Option<Point> {
    match dir {
      Direction::Up if y == 0 => None,
      Direction::Up => Some((x, y - 1)),
      Direction::Down if y == self.height() - 1 => None,
      Direction::Down => Some((x, y + 1)),
      Direction::Left if x == 0 => None,
      Direction::Left => Some((x - 1, y)),
      Direction::Right if x == self.width() - 1 => None,
      Direction::Right => Some((x + 1, y)),
    }
  }

  fn travel(&self, init: Beam) -> usize {
    let mut visited = HashSet::<Beam>::new();
    let mut beams = vec![init];
    while let Some(beam) = beams.pop() {
      let cell = &self.cells[beam.pos.1 as usize][beam.pos.0 as usize];
      for dir in cell.redir(beam.dir) {
        if let Some(pos) = self.advance(dir, beam.pos) {
          let new = Beam { pos, dir };
          if !visited.contains(&new) { beams.push(new); }
        }
      }
      visited.insert(beam);
    }
    visited.into_iter().map(|x| x.pos).collect::<HashSet<Point>>().len()
  }

  fn travel_all(&self) -> usize {
    let it1 = (0..self.width()).flat_map(|x| [
      Beam { pos: (x, 0), dir: Direction::Down },
      Beam { pos: (x, self.height() - 1), dir: Direction::Up },
    ]);
    let it2 = (0..self.height()).flat_map(|y| [
      Beam { pos: (0, y), dir: Direction::Right },
      Beam { pos: (self.width() - 1, y), dir: Direction::Left },
    ]);
    it1.chain(it2).map(|b| self.travel(b)).max().unwrap()
  }
}

pub fn run(content: &str) {
  let field = Field::parse(content);
  let res1 = field.travel(Beam { pos: (0, 0), dir: Direction::Right });
  let res2 = field.travel_all();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

  #[test]
  fn small() {
    let test = super::Field::parse(TEST.trim());
    let init = super::Beam { pos: (0, 0), dir: super::Direction::Right };
    assert_eq!(test.travel(init), 46);
  }

  #[test]
  fn large() {
    let test = super::Field::parse(TEST.trim());
    assert_eq!(test.travel_all(), 51);
  }
}
