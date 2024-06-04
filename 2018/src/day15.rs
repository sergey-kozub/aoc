use std::collections::{HashSet, VecDeque};
use std::fmt;

type Coord = (usize, usize);
const DEAD: Coord = (0, 0);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Race {
  Elf,
  Goblin,
}

#[derive(Clone, Debug)]
struct Unit {
  race: Race,
  power: i32,
  hp: i32,
}

#[derive(Clone, Debug)]
enum Tile {
  Open,
  Wall,
  Unit(Unit),
}

struct Game {
  tiles: Vec<Vec<Tile>>,
  units: Vec<Coord>,
}

impl Unit {
  fn new(race: Race, power: i32) -> Self {
    Unit { race, power, hp: 200 }
  }
}

impl Game {
  fn parse(text: &str, power: (i32, i32)) -> Self {
    let tiles = text.lines().map(|line| {
      line.chars().map(|c| match c {
        '.' => Tile::Open,
        '#' => Tile::Wall,
        'E' => Tile::Unit(Unit::new(Race::Elf, power.0)),
        'G' => Tile::Unit(Unit::new(Race::Goblin, power.1)),
        _ => panic!("unknown symbol"),
      }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let units = tiles.iter().enumerate().flat_map(|(y, a)| {
      a.iter().enumerate().filter_map(move |(x, t)| {
        if matches!(t, Tile::Unit(_)) {Some((y, x))} else {None}
      })
    }).collect::<Vec<_>>();
    Game { tiles, units }
  }

  fn get_unit(&self, (y, x): Coord) -> &Unit {
    match &self.tiles[y][x] {
      Tile::Unit(unit) => unit,
      _ => panic!("No unit @({y},{x})"),
    }
  }

  fn attack_target(&self, pos: Coord) -> Option<Coord> {
    let unit = self.get_unit(pos);
    let targets = adjacent(pos).into_iter().filter_map(|(y, x)| {
      match &self.tiles[y][x] {
        Tile::Unit(other) if other.race != unit.race => Some((y, x)),
        _ => None,
      }
    }).collect::<Vec<_>>();
    if targets.is_empty() { return None; }
    let min_hp = targets.iter().map(|&p| self.get_unit(p).hp).min().unwrap();
    targets.into_iter().filter(|&p| self.get_unit(p).hp == min_hp).next()
  }

  fn move_target(&self, pos: Coord) -> Option<Coord> {
    let unit = self.get_unit(pos);
    let targets = self.units.iter().filter(|&&p| {
      p != DEAD && self.get_unit(p).race != unit.race
    }).flat_map(|&p| adjacent(p).into_iter()).collect::<HashSet<_>>();
    if targets.is_empty() { return None; }
    self.search(pos, &targets)
  }

  fn search(&self, init: Coord, targets: &HashSet<Coord>) -> Option<Coord> {
    if targets.contains(&init) { return Some(init); }
    let mut step = 0_usize;
    let mut queue = VecDeque::from([(init, step)]);
    let mut visited = HashSet::<Coord>::new();
    while let Some((p, n)) = queue.pop_front() {
      if n > step {
        if visited.intersection(&targets).next().is_some() { break; }
        step = n;
      }
      for next in adjacent(p) {
        if !matches!(self.tiles[next.0][next.1], Tile::Open)
          || visited.contains(&next) { continue; }
        queue.push_back((next, n + 1));
        visited.insert(next);
      }
    }
    let mut valid = visited.intersection(&targets).collect::<Vec<_>>();
    valid.sort();
    valid.into_iter().next().copied()
  }

  fn get_winner(&self) -> Option<Race> {
    let race = self.get_unit(self.units[0]).race;
    let same = self.units.iter().all(|&p| self.get_unit(p).race == race);
    if same {Some(race)} else {None}
  }

  fn do_attack(&mut self, pos: Coord, to: Coord) -> bool {
    let power = self.get_unit(pos).power;
    if let Tile::Unit(target) = &mut self.tiles[to.0][to.1] {
      target.hp -= power;
      if target.hp <= 0 {
        self.tiles[to.0][to.1] = Tile::Open;
        self.update(to, DEAD);
        return true;
      }
    }
    false
  }

  fn do_move(&mut self, pos: Coord, to: Coord) -> Coord {
    let targets = adjacent(pos).into_iter().collect::<HashSet<_>>();
    let next = self.search(to, &targets).unwrap();
    let unit = self.tiles[pos.0][pos.1].clone();
    self.tiles[pos.0][pos.1] = Tile::Open;
    self.tiles[next.0][next.1] = unit;
    self.update(pos, next);
    next
  }

  fn update(&mut self, pos: Coord, new: Coord) {
    let idx = self.units.iter().position(|&p| p == pos).unwrap();
    self.units[idx] = new;
  }

  fn action(&mut self, mut pos: Coord) -> bool {
    let mut target = self.attack_target(pos);
    if target.is_none() {
      if let Some(to) = self.move_target(pos) {
        pos = self.do_move(pos, to);
        target = self.attack_target(pos);
      }
    }
    if let Some(to) = target {
      return self.do_attack(pos, to);
    }
    false
  }

  fn action_round(&mut self) -> bool {
    let mut last_kill: Option<usize> = None;
    for i in 0..self.units.len() {
      let pos = self.units[i];
      if pos == DEAD { continue; }
      let kill = self.action(pos);
      if kill { last_kill = Some(i); }
    }
    let partial = match last_kill {
      Some(i) => self.units.iter().skip(i + 1).any(|&p| p != DEAD),
      None => true,
    };
    self.units.retain(|&p| p != DEAD);
    self.units.sort();
    self.get_winner().is_none() || !partial
  }

  fn play(&mut self) -> usize {
    let mut rounds = 0_usize;
    while self.action_round() { rounds += 1; }
    rounds
  }

  fn play_and_score(&mut self) -> usize {
    let rounds = self.play();
    let sum_hp = self.units.iter().map(|&p| self.get_unit(p).hp).sum::<i32>();
    rounds * sum_hp as usize
  }

  fn elf_count(&self) -> usize {
    self.units.iter().filter(|&&p| self.get_unit(p).race == Race::Elf).count()
  }

  fn elf_power_score(text: &str) -> (i32, usize) {
    let (mut l, mut r) = (0, 200);
    let mut score = 0;
    while l + 1 < r {
      let m = (l + r) / 2;
      let mut game = Game::parse(text, (m, 3));
      let elves = game.elf_count();
      let value = game.play_and_score();
      if game.elf_count() == elves {
        r = m; score = value;
      } else {
        l = m;
      }
    }
    (r, score)
  }
}

impl fmt::Debug for Game {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let race = |unit: &Unit| match unit.race {
      Race::Elf => 'E',
      Race::Goblin => 'G',
    };
    for row in &self.tiles {
      let line = row.iter().map(|tile| match tile {
        Tile::Open => '.',
        Tile::Wall => '#',
        Tile::Unit(unit) => race(unit),
      }).collect::<String>();
      writeln!(f, "{}", line)?;
    }
    let extra = self.units.iter().map(|&p| {
      let unit = self.get_unit(p);
      format!("{}({})", race(unit), unit.hp)
    }).collect::<Vec<_>>();
    writeln!(f, "{}", extra.join(", "))
  }
}

fn adjacent((y, x): Coord) -> [Coord; 4] {
  [(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)]
}

pub fn run(content: &str) {
  let res1 = Game::parse(content, (3, 3)).play_and_score();
  let res2 = Game::elf_power_score(content).1;
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
  const TEST_2: &str = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
  const TEST_3: &str = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
  const TEST_4: &str = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
  const TEST_5: &str = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

  #[test]
  fn small() {
    let score = |s: &str| super::Game::parse(s, (3, 3)).play_and_score();
    assert_eq!(score(TEST_1), 36334);
    assert_eq!(score(TEST_2), 39514);
    assert_eq!(score(TEST_3), 27755);
    assert_eq!(score(TEST_4), 28944);
    assert_eq!(score(TEST_5), 18740);
  }

  #[test]
  fn large() {
    let score = |s: &str| super::Game::elf_power_score(s);
    assert_eq!(score(TEST_2), (4, 31284));
    assert_eq!(score(TEST_3), (15, 3478));
    assert_eq!(score(TEST_4), (12, 6474));
    assert_eq!(score(TEST_5), (34, 1140));
  }
}
