use std::cmp;

#[derive(Debug, Default, PartialEq, PartialOrd)]
struct CubeSet {
  red: u32,
  green: u32,
  blue: u32,
}

#[derive(Debug)]
struct CubeGame {
  id: u32,
  sets: Vec<CubeSet>,
}

impl CubeSet {
  fn parse(text: &str) -> CubeSet {
    let mut result = CubeSet::default();
    for item in text.split(", ") {
      let (s1, s2) = item.split_once(' ').unwrap();
      let n = s1.parse::<u32>().unwrap();
      match s2 {
        "red" => result.red = n,
        "green" => result.green = n,
        "blue" => result.blue = n,
        _ => panic!("unknown color"),
      }
    }
    result
  }

  fn within(&self, limit: &CubeSet) -> bool {
    self.red <= limit.red &&
    self.green <= limit.green &&
    self.blue <= limit.blue
  }
}

impl CubeGame {
  fn parse(text: &str) -> CubeGame {
    let (s1, s2) = text.split_once(": ").unwrap();
    let id = s1.split(' ').nth(1).unwrap().parse::<u32>().unwrap();
    let sets = s2.split("; ").map(CubeSet::parse).collect();
    CubeGame { id, sets }
  }

  fn within(&self, limit: &CubeSet) -> bool {
    self.sets.iter().all(|x| x.within(limit))
  }

  fn power(&self) -> u32 {
    let (mut r, mut g, mut b) = (0, 0, 0);
    for set in &self.sets {
      r = cmp::max(r, set.red);
      g = cmp::max(g, set.green);
      b = cmp::max(b, set.blue);
    }
    r * g * b
  }
}

pub fn run(content: &str) {
  let games: Vec<CubeGame> = content.lines().map(CubeGame::parse).collect();
  let limit = CubeSet { red: 12, green: 13, blue: 14 };
  let res1: u32 = games.iter().filter(|x| x.within(&limit)).map(|x| x.id).sum();
  let res2: u32 = games.iter().map(|x| x.power()).sum();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

  #[test]
  fn small() {
    let games = TEST.lines().map(super::CubeGame::parse);
    let limit = super::CubeSet { red: 12, green: 13, blue: 14 };
    let test: Vec<bool> = games.map(|x| x.within(&limit)).collect();
    assert_eq!(test, [true, true, false, false, true]);
  }

  #[test]
  fn large() {
    let games = TEST.lines().map(super::CubeGame::parse);
    let test: Vec<u32> = games.map(|x| x.power()).collect();
    assert_eq!(test, [48, 12, 1560, 630, 36]);
  }
}
