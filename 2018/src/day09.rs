
#[derive(Debug)]
struct Marble {
  number: u32,
  left: usize,
  right: usize,
}

#[derive(Debug)]
struct Board {
  items: Vec<Marble>,
  current: usize,
}

#[derive(Debug)]
struct Game {
  players: usize,
  points: u32,
}

impl Board {
  fn new() -> Board {
    let init = Marble { number: 0, left: 0, right: 0 };
    Board { items: vec![init], current: 0 }
  }

  fn add(&mut self, number: u32) {
    let l = self.items[self.current].right;
    let r = self.items[l].right;
    let n = self.items.len();
    self.items.push(Marble { number, left: l, right: r });
    self.items[l].right = n;
    self.items[r].left = n;
    self.current = n;
  }

  fn remove(&mut self) -> u32 {
    let i = (0..7).fold(self.current, |p, _| self.items[p].left);
    let number = self.items[i].number;
    let (l, r) = (self.items[i].left, self.items[i].right);
    self.items[l].right = r;
    self.items[r].left = l;
    self.current = r;
    number
  }
}

impl Game {
  fn parse(text: &str) -> Game {
    let a = text.split(' ').collect::<Vec<_>>();
    let players = a[0].parse::<usize>().unwrap();
    let points = a[6].parse::<u32>().unwrap();
    Game { players, points }
  }

  fn play(&self, k: u32) -> u32 {
    let mut board = Board::new();
    let mut scores = vec![0_u32; self.players];
    (1..=k*self.points).for_each(|i| {
      if i % 23 != 0 {
        board.add(i);
      } else {
        let score = i + board.remove();
        scores[i as usize % self.players] += score;
      }
    });
    scores.into_iter().max().unwrap()
  }
}

pub fn run(content: &str) {
  let game = Game::parse(content);
  let res1 = game.play(1);
  let res2 = game.play(100);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "10 players; last marble is worth 1618 points";
  const TEST_2: &str = "13 players; last marble is worth 7999 points";
  const TEST_3: &str = "17 players; last marble is worth 1104 points";
  const TEST_4: &str = "21 players; last marble is worth 6111 points";
  const TEST_5: &str = "30 players; last marble is worth 5807 points";

  #[test]
  fn small() {
    assert_eq!(super::Game::parse(TEST_1).play(1), 8317);
    assert_eq!(super::Game::parse(TEST_2).play(1), 146373);
    assert_eq!(super::Game::parse(TEST_3).play(1), 2764);
    assert_eq!(super::Game::parse(TEST_4).play(1), 54718);
    assert_eq!(super::Game::parse(TEST_5).play(1), 37305);
  }
}
