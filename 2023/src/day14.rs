use std::collections::{HashMap, HashSet};
use std::fmt;

type Point = (usize, usize);

#[derive(Clone)]
struct Field {
  rock: HashSet<Point>,
  roll: HashSet<Point>,
  size: Point,
}

impl Field {
  fn parse(text: &str) -> Field {
    let width = text.lines().next().unwrap().len();
    let height = text.lines().count();
    let collect = |symb| text.lines().enumerate().flat_map(
      |(y, s)| s.chars().enumerate().filter_map(move
      |(x, c)| if c == symb {Some((x, y))} else {None}
    )).collect::<HashSet<Point>>();
    Field {
      rock: collect('#'),
      roll: collect('O'),
      size: (width, height),
    }
  }

  fn total_load(&self) -> usize {
    self.roll.iter().map(|(_, y)| self.size.1 - y).sum()
  }

  fn vertical(&self, x: usize, it: &mut dyn Iterator<Item=usize>) -> usize {
    it.take_while(|&i| !self.rock.contains(&(x, i)))
      .filter(|&i| !self.roll.contains(&(x, i))).count()
  }

  fn horizontal(&self, y: usize, it: &mut dyn Iterator<Item=usize>) -> usize {
    it.take_while(|&i| !self.rock.contains(&(i, y)))
      .filter(|&i| !self.roll.contains(&(i, y))).count()
  }

  fn tilt_north(self) -> Field {
    let roll = self.roll.iter().map(|&(x, y)| {
      (x, y - self.vertical(x, &mut (0..y).rev()))
    }).collect();
    Field { rock: self.rock, roll, size: self.size }
  }

  fn tilt_south(self) -> Field {
    let roll = self.roll.iter().map(|&(x, y)| {
      (x, y + self.vertical(x, &mut (y+1..self.size.1)))
    }).collect();
    Field { rock: self.rock, roll, size: self.size }
  }

  fn tilt_west(self) -> Field {
    let roll = self.roll.iter().map(|&(x, y)| {
      (x - self.horizontal(y, &mut (0..x).rev()), y)
    }).collect();
    Field { rock: self.rock, roll, size: self.size }
  }

  fn tilt_east(self) -> Field {
    let roll = self.roll.iter().map(|&(x, y)| {
      (x + self.horizontal(y, &mut (x+1..self.size.0)), y)
    }).collect();
    Field { rock: self.rock, roll, size: self.size }
  }

  fn full_cycle(self) -> Field {
    self.tilt_north().tilt_west().tilt_south().tilt_east()
  }

  fn state(&self) -> String {
    String::from_utf8(
      (0..self.size.1).flat_map(|y| (0..self.size.0).map(move |x|
        (if self.roll.contains(&(x, y)) {'x'} else {'.'}) as u8
    )).collect::<Vec<u8>>()).unwrap()
  }

  fn run(self, count: usize) -> Field {
    let mut result = self;
    let mut visited = HashMap::<String, usize>::new();
    for step in 0..count {
      let state = result.state();
      if let Some(prev) = visited.get(&state) {
        let rest = (count - step) % (step - prev);
        return (0..rest).fold(result, |x, _| x.full_cycle());
      }
      visited.insert(state, step);
      result = result.full_cycle();
    }
    result
  }
}

impl fmt::Debug for Field {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", (0..self.size.1).fold(String::new(), |s, y| {
      s + &String::from_utf8((0..self.size.0).map(|x| (
        if self.rock.contains(&(x, y)) {'#'} else
        if self.roll.contains(&(x, y)) {'O'} else {'.'}
      ) as u8).collect::<Vec<_>>()).unwrap() + "\n"
    }))
  }
}

pub fn run(content: &str) {
  let field = Field::parse(content);
  let res1 = field.clone().tilt_north().total_load();
  let res2 = field.run(1_000_000_000).total_load();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

  #[test]
  fn small() {
    let field = super::Field::parse(TEST);
    assert_eq!(field.tilt_north().total_load(), 136);
  }

  #[test]
  fn large() {
    let field = super::Field::parse(TEST);
    assert_eq!(field.run(1_000_000_000).total_load(), 64);
  }
}
