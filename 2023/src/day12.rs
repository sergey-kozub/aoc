use std::collections::HashMap;
use std::iter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
  Operational,
  Damaged,
  Unknown,
}

#[derive(Debug)]
struct Spring {
  cells: Vec<Cell>,
  groups: Vec<usize>,
}

#[derive(Default)]
struct Solution {
  cache: HashMap<(usize, usize, bool), usize>,
}

impl Spring {
  fn parse(text: &str) -> Spring {
    let (s1, s2) = text.split_once(' ').unwrap();
    let cells = s1.chars().map(|c| match c {
      '.' => Cell::Operational,
      '#' => Cell::Damaged,
      '?' => Cell::Unknown,
      _ => panic!("unknown symbol"),
    }).collect::<Vec<Cell>>();
    let groups = s2.split(',').map(|s| {
      s.parse::<usize>().unwrap()
    }).collect::<Vec<usize>>();
    Spring { cells, groups }
  }

  fn unfold(&self, count: usize) -> Spring {
    let concat = || iter::once(Cell::Unknown).chain(self.cells.iter().cloned());
    let cells = iter::repeat_with(concat)
      .take(count).flatten().skip(1).collect::<Vec<Cell>>();
    let groups = iter::repeat(self.groups.iter().cloned())
      .take(count).flatten().collect::<Vec<usize>>();
    Spring { cells, groups }
  }

  fn arrangements(&self) -> usize {
    let mut solution = Solution::default();
    solution.descend(&self.cells[..], &self.groups[..], false)
  }
}

impl Solution {
  fn descend(&mut self, cells: &[Cell], groups: &[usize],
             restrict: bool) -> usize {
    let some_is = |a: &[Cell], x: Cell| a.iter().any(|&c| c == x);
    if groups.is_empty() { return !some_is(cells, Cell::Damaged) as usize; }
    let next = groups[0];
    if next > cells.len() { return 0; }

    let key = (cells.len(), groups.len(), restrict);
    if let Some(res) = self.cache.get(&key) { return *res; }
    let mut res = 0_usize;
    if !matches!(cells[0], Cell::Damaged) {
      res += self.descend(&cells[1..], groups, false);
    }
    if !matches!(cells[0], Cell::Operational) && !restrict &&
       !some_is(&cells[..next], Cell::Operational) {
      res += self.descend(&cells[next..], &groups[1..], true);
    }
    self.cache.insert(key, res);
    res
  }
}

pub fn run(content: &str) {
  let springs: Vec<Spring> = content.lines().map(Spring::parse).collect();
  let unfolded: Vec<Spring> = springs.iter().map(|x| x.unfold(5)).collect();
  let count = |x: &[Spring]| x.iter().map(|v| v.arrangements()).sum::<usize>();
  println!("{} {}", count(&springs), count(&unfolded));
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

  #[test]
  fn small() {
    let test = TEST.lines().map(super::Spring::parse)
      .map(|x| x.arrangements()).collect::<Vec<usize>>();
    assert_eq!(test, [1, 4, 1, 1, 4, 10]);
  }

  #[test]
  fn large() {
    let test = TEST.lines().map(super::Spring::parse)
      .map(|x| x.unfold(5))
      .map(|x| x.arrangements()).collect::<Vec<usize>>();
    assert_eq!(test, [1, 16384, 1, 16, 2500, 506250]);
  }
}
