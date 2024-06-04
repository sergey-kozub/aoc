use std::cmp;

#[derive(Clone, Eq, PartialEq)]
enum Cell {
  Empty,
  Rock,
}

struct Pattern {
  data: Vec<Vec<Cell>>,
  ignore: Option<usize>,
}

impl Pattern {
  fn parse(text: &str) -> Pattern {
    let data = text.lines().map(|s|
      s.chars().map(|c| match c {
        '.' => Cell::Empty,
        '#' => Cell::Rock,
        _ => panic!("unknown symbol"),
      }).collect()
    ).collect();
    Pattern { data, ignore: None }
  }

  fn width(&self) -> usize { self.data[0].len() }
  fn height(&self) -> usize { self.data.len() }

  fn horizontal(&self) -> Option<usize> {
    (1..self.height()).filter_map(|i| {
      let n = cmp::min(i, self.height() - i);
      let valid = (0..n).all(|d| self.data[i-d-1] == self.data[i+d]) &&
        self.ignore != Some(i * 100);
      if valid {Some(i)} else {None}
    }).next()
  }

  fn vertical(&self) -> Option<usize> {
    (1..self.width()).filter_map(|i| {
      let n = cmp::min(i, self.width() - i);
      let valid = (0..n).all(|d| {
        (0..self.height()).all(|j| self.data[j][i-d-1] == self.data[j][i+d])
      }) && self.ignore != Some(i);
      if valid {Some(i)} else {None}
    }).next()
  }

  fn score(&self) -> Option<usize> {
    self.vertical().or_else(|| self.horizontal().map(|t| t * 100))
  }

  fn smudge(&self, x: usize, y: usize) -> Pattern {
    let mut data = self.data.clone();
    data[y][x] = match data[y][x] {
      Cell::Empty => Cell::Rock,
      Cell::Rock => Cell::Empty,
    };
    Pattern { data, ignore: self.score() }
  }

  fn smudge_all(&self) -> Option<Pattern> {
    (0..self.height()).flat_map(|y|
      (0..self.width()).map(move |x| self.smudge(x, y))
    ).filter(|p| p.score().is_some()).next()
  }
}

pub fn run(content: &str) {
  let patterns: Vec<Pattern> = content.split("\n\n")
    .map(Pattern::parse).collect();
  let res1 = patterns.iter().map(
    |p| p.score().unwrap()).sum::<usize>();
  let res2 = patterns.iter().map(
    |p| p.smudge_all().and_then(|t| t.score()).unwrap()).sum::<usize>();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
  const TEST_2: &str = "\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

  #[test]
  fn small() {
    let parse = |s| super::Pattern::parse(s);
    assert_eq!(parse(TEST_1).vertical(), Some(5));
    assert_eq!(parse(TEST_2).horizontal(), Some(4));
  }

  #[test]
  fn large() {
    let parse = |s| super::Pattern::parse(s).smudge_all().unwrap();
    assert_eq!(parse(TEST_1).horizontal(), Some(3));
    assert_eq!(parse(TEST_2).horizontal(), Some(1));
  }
}
