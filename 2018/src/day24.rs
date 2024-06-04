use regex::{Match, Regex};
use std::cmp;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Team {
  ImmuneSystem,
  Infection,
}

#[derive(Clone, Debug)]
struct Army {
  team: Team,
  units: u32,
  hp: u32,
  initiative: u32,
  damage: u32,
  attack: String,
  weak: HashSet<String>,
  immune: HashSet<String>,
}

#[derive(Clone, Debug)]
struct Game {
  data: Vec<Army>,
}

impl Army {
  fn parse(text: &str, team: Team) -> Army {
    let re = Regex::new(concat!(
      r"(\d+) units? each with (\d+) hit points?( \((.+)\))?",
      r" with an attack that does (\d+) (\w+) damage",
      r" at initiative (\d+)")).unwrap();
    let m = re.captures(text).unwrap();
    let parse_num = |s: &str| s.parse::<u32>().unwrap();
    let parse_set = |item: Option<Match>, prefix: &str| {
      item.and_then(|m| m.as_str().split("; ")
        .filter(|&s| s.starts_with(prefix)).next())
        .map(|s| s.split_once(" to ").unwrap().1)
        .map(|s| HashSet::from_iter(s.split(", ").map(String::from)))
        .unwrap_or(HashSet::new())
    };
    Army {
      team,
      units: parse_num(&m[1]),
      hp: parse_num(&m[2]),
      initiative: parse_num(&m[7]),
      damage: parse_num(&m[5]),
      attack: String::from(&m[6]),
      weak: parse_set(m.get(4), "weak"),
      immune: parse_set(m.get(4), "immune"),
    }
  }

  fn power(&self) -> u32 {
    self.units * self.damage
  }

  fn calc_damage(&self, to: &Army) -> u32 {
    if to.immune.contains(&self.attack) {0}
    else if to.weak.contains(&self.attack) {self.power() * 2}
    else {self.power()}
  }

  fn calc_kills(&self, to: &Army) -> u32 {
    cmp::min(self.calc_damage(to) / to.hp, to.units)
  }
}

impl Game {
  fn parse(text: &str) -> Self {
    let (s1, s2) = text.split_once("\n\n").unwrap();
    let it1 = s1.lines().skip(1).map(|s| Army::parse(s, Team::ImmuneSystem));
    let it2 = s2.lines().skip(1).map(|s| Army::parse(s, Team::Infection));
    Game { data: Vec::from_iter(it1.chain(it2)) }
  }

  fn boost(&self, value: u32) -> Self {
    let mut result = self.clone();
    for army in result.data.iter_mut() {
      if army.team == Team::ImmuneSystem {
        army.damage += value;
      }
    }
    result
  }

  fn select(&self) -> Vec<Option<usize>> {
    let n = self.data.len();
    let mut selected = vec![false; n];
    self.data.iter().map(|army| {
      let mut targets = (0..n).filter_map(|i| {
        let t = &self.data[i];
        if t.team != army.team && !t.immune.contains(&army.attack)
          && !selected[i] {Some(i)} else {None}
      }).collect::<Vec<_>>();
      targets.sort_by_key(|&i| {
        let t = &self.data[i];
        (army.calc_damage(t), t.power(), t.initiative)
      });
      targets.last().map(|&i| { selected[i] = true; i })
    }).collect()
  }

  fn attack(&mut self) -> bool {
    let inv = |x: u32| -(x as i32);
    self.data.sort_by_key(|x| (inv(x.power()), inv(x.initiative)));
    let order = self.select();
    let mut index = self.data.iter().enumerate()
      .map(|(k, v)| (inv(v.initiative), k)).collect::<Vec<_>>();
    index.sort();

    let mut changed = false;
    for (_, i) in index {
      if let Some(j) = order[i] {
        let kills = self.data[i].calc_kills(&self.data[j]);
        self.data[j].units -= kills;
        changed |= kills > 0;
      }
    }
    self.data.retain(|x| x.units > 0);
    changed
  }

  fn play(&mut self) -> (Option<Team>, u32) {
    while self.attack() {}
    let first = self.data[0].team;
    if self.data.iter().any(|x| x.team != first) { return (None, 0); }
    let score = self.data.iter().map(|x| x.units).sum();
    (Some(first), score)
  }
}

fn find_boost(game: &Game, limit: u32) -> (u32, u32) {
  let (mut l, mut r) = (0, limit);
  let mut score = 0;
  while l + 1 < r {
    let m = (l + r) / 2;
    let (wins, result) = game.boost(m).play();
    if matches!(wins, Some(Team::ImmuneSystem))
      { r = m; score = result; } else { l = m; }
  }
  (r, score)
}

pub fn run(content: &str) {
  let game = Game::parse(content);
  let res1 = game.clone().play().1;
  let res2 = find_boost(&game, 1_000_000).1;
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with
  an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning,
  slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack
  that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire,
  cold) with an attack that does 12 slashing damage at initiative 4";

  #[test]
  fn small() {
    let clean = String::from(TEST).replace("\n ", "");
    let mut test = super::Game::parse(&clean);
    assert_eq!(test.play(), (Some(super::Team::Infection), 5216));
  }

  #[test]
  fn large() {
    let clean = String::from(TEST).replace("\n ", "");
    let test = super::Game::parse(&clean);
    assert_eq!(super::find_boost(&test, 10_000), (1570, 51));
  }
}
