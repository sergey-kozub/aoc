use std::fmt;

struct Game {
  digits: Vec<u8>,
  index: [usize; 2],
}

impl Game {
  fn new() -> Game {
    Game {
      digits: vec![3, 7],
      index: [0, 1],
    }
  }

  fn advance(&mut self, pos: usize) {
    let cur = &mut self.index[pos];
    *cur = (*cur + self.digits[*cur] as usize + 1) % self.digits.len();
  }

  fn step(&mut self) {
    let sum = self.digits[self.index[0]] + self.digits[self.index[1]];
    if sum >= 10 { self.digits.push(sum / 10); }
    self.digits.push(sum % 10);
    for i in 0..2 { self.advance(i); }
  }

  fn score(&mut self, size: usize) -> String {
    while self.digits.len() < size + 10 { self.step(); }
    (size..size+10).map(|i| (self.digits[i] + 48) as char).collect::<String>()
  }

  fn search(&mut self, value: String) -> usize {
    let size = value.len();
    while self.digits.len() < size { self.step(); }
    loop {
      self.step();
      for offset in [0, 1] {
        let i = self.digits.len() - size - offset;
        if value.chars().enumerate().all(|(j, c)| {
          c == (self.digits[i + j] + 48) as char
        }) { return i; }
      }
    }
  }
}

impl fmt::Debug for Game {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.digits.iter().enumerate().fold(String::new(), |s, t| {
      let (mut c1, mut c2) = (" ", " ");
      if t.0 == self.index[0] { (c1, c2) = ("(", ")"); }
      if t.0 == self.index[1] { (c1, c2) = ("[", "]"); }
      s + c1 + &t.1.to_string() + c2
    }))
  }
}

pub fn run(content: &str) {
  let size = content.trim().parse::<usize>().unwrap();
  let res1 = Game::new().score(size);
  let res2 = Game::new().search(size.to_string());
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn small() {
    let mut test = super::Game::new();
    assert_eq!(test.score(9), "5158916779");
    assert_eq!(test.score(5), "0124515891");
    assert_eq!(test.score(18), "9251071085");
    assert_eq!(test.score(2018), "5941429882");
  }

  #[test]
  fn large() {
    let search = |n: &str| super::Game::new().search(String::from(n));
    assert_eq!(search("51589"), 9);
    assert_eq!(search("01245"), 5);
    assert_eq!(search("92510"), 18);
    assert_eq!(search("59414"), 2018);
  }
}
