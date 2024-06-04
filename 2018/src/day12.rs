use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
struct Tunnel {
  start: isize,
  pots: VecDeque<bool>,
  patterns: Vec<bool>,
}

impl Tunnel {
  fn parse(text: &str) -> Tunnel {
    let (s1, s2) = text.split_once("\n\n").unwrap();
    let (_, s0) = s1.split_once(": ").unwrap();
    let pots = s0.chars().map(|c| c == '#').collect::<VecDeque<_>>();
    let mut patterns = vec![false; 32];
    s2.lines().for_each(|s| {
      let (s1, s2) = s.split_once(" => ").unwrap();
      let index = s1.chars().enumerate().map(|(k, v)| {
        if v == '#' {1 << k} else {0}
      }).sum::<usize>();
      if s2 == "#" { patterns[index] = true; }
    });
    Tunnel { start: 0, pots, patterns }
  }

  fn state(&self) -> String {
    self.pots.iter().map(|&v| if v {'#'} else {'.'}).collect::<String>()
  }

  fn grow(&mut self) -> String {
    let last = self.start + self.pots.len() as isize - 1;
    let mut next = VecDeque::<bool>::new();
    let mut start = self.start - 2;
    for i in start..=last+2 {
      let index = (i-2..=i+2).map(|j| {
        if j < self.start || j > last { return 0; }
        if self.pots[(j - self.start) as usize] {1 << (j - i + 2)} else {0}
      }).sum::<usize>();
      next.push_back(self.patterns[index]);
    }
    while !next[0] {
      next.pop_front();
      start += 1;
    }
    while !next.back().unwrap() {
      next.pop_back();
    }
    self.start = start;
    self.pots = next;
    self.state()
  }

  fn grow_n(&mut self, count: usize) -> isize {
    let mut hist = HashMap::<String, (usize, isize)>::new();
    for step in 1..=count {
      let key = self.grow();
      if let Some((n, s)) = hist.get(&key) {
        assert_eq!(step, n + 1);
        self.start += (count - step) as isize * (self.start - s);
        break;
      }
      hist.insert(key, (step, self.start));
    }
    self.pots.iter().enumerate().map(|(k, v)| {
      if *v {self.start + k as isize} else {0}
    }).sum::<isize>()
  }
}

pub fn run(content: &str) {
  let mut tunnel = Tunnel::parse(content);
  let res1 = tunnel.clone().grow_n(20);
  let res2 = tunnel.grow_n(50_000_000_000);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

  #[test]
  fn small() {
    let mut test = super::Tunnel::parse(TEST);
    assert_eq!(test.grow_n(20), 325);
  }
}
