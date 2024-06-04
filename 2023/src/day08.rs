use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;

#[derive(Clone, Copy, Debug)]
enum Move {
  Left,
  Right,
}

#[derive(Debug)]
struct Node {
  left: String,
  right: String,
}

#[derive(Debug)]
struct Game {
  moves: Vec<Move>,
  nodes: HashMap<String, Node>,
}

impl Game {
  fn parse(text: &str) -> Game {
    let (s1, s2) = text.split_once("\n\n").unwrap();
    let moves: Vec<Move> = s1.chars().map(|c| match c {
      'L' => Move::Left,
      'R' => Move::Right,
      _ => panic!("incorrect move"),
    }).collect();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let nodes: HashMap<String, Node> = re.captures_iter(s2).map(|c| {
      let get = |i: usize| String::from(&c[i]);
      (get(1), Node { left: get(2), right: get(3) })
    }).collect();
    Game { moves, nodes }
  }

  fn advance_one(&self, cur: &str, dir: Move) -> &str {
    let node = &self.nodes[cur];
    match dir {
      Move::Left => &node.left,
      Move::Right => &node.right,
    }
  }

  fn count_steps(&self) -> usize {
    let mut cur = "AAA";
    repeat(&self.moves).flatten().take_while(|&dir| {
      cur = self.advance_one(cur, *dir);
      cur != "ZZZ"
    }).count() + 1
  }

  fn advance_all<'a>(&'a self, cur: &'a str) -> &str {
    self.moves.iter().fold(cur, |a, &b| self.advance_one(a, b))
  }

  fn find_loop(&self, start: &str) -> usize {
    let mut pos = HashSet::<&str>::new();
    let mut cur = self.advance_all(start);
    while !pos.contains(&cur) {
      pos.insert(cur);
      cur = self.advance_all(cur);
    }
    pos.len()
  }

  fn find_exits(&self, start: &str, loops: usize) -> HashSet<usize> {
    let mut cur = start;
    repeat(&self.moves).take(loops).flatten().enumerate().filter_map(|(k, v)| {
      let is_exit = cur.ends_with('Z');
      cur = self.advance_one(cur, *v);
      if is_exit {Some(k)} else {None}
    }).collect()
  }

  fn count_multiple(&self) -> usize {
    let n = self.moves.len();
    let s = self.nodes.keys()
      .filter(|k| k.ends_with('A'))
      .map(|k| (self.advance_all(k), self.find_loop(k)))
      .map(|(s, n)| (n, self.find_exits(s, n)))
      .reduce(|a, b| {
        let (size_a, size_b) = (a.0 * n, b.0 * n);
        let res = repeat(a.1).take(b.0).enumerate()
          .map(|(k, v)| v.into_iter().map(move |x| x + k * size_a)).flatten()
          .filter(|x| b.1.contains(&(x % size_b)));
        (a.0 * b.0, res.collect())
      }).unwrap().1;
    s.iter().min().unwrap() + n
  }
}

pub fn run(content: &str) {
  let game = Game::parse(content);
  println!("{} {}", game.count_steps(), game.count_multiple());
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "RL\n
AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
  const TEST_2: &str = "LLR\n
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
  const TEST_3: &str = "LR\n
11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

  #[test]
  fn small() {
    assert_eq!(super::Game::parse(TEST_1).count_steps(), 2);
    assert_eq!(super::Game::parse(TEST_2).count_steps(), 6);
  }

  #[test]
  fn large() {
    assert_eq!(super::Game::parse(TEST_3).count_multiple(), 6);
  }
}
