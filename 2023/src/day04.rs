use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Card {
  winning: HashSet<u32>,
  present: HashSet<u32>,
}

impl Card {
  fn new(data: &str) -> Card {
    let (_, s0) = data.split_once(": ").unwrap();
    let (s1, s2) = s0.split_once(" | ").unwrap();
    let parse = |s: &str| -> HashSet<u32> {
      s.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()
    };
    Card {
      winning: parse(s1),
      present: parse(s2),
    }
  }

  fn score(&self) -> u32 {
    let win = self.winning.intersection(&self.present).count();
    2_u32.pow(win as u32) / 2
  }

  fn roll(cards: &[Card]) -> u32 {
    let mut result = 0_u32;
    let mut owned = VecDeque::from(vec![1_u32; cards.len()]);
    for card in cards {
      let m = match owned.pop_front() {
        Some(n) => n,
        None => break,
      };
      let win = card.winning.intersection(&card.present).count();
      for i in 0..win {
        match owned.get_mut(i) {
          Some(value) => *value += m,
          None => owned.push_back(m),
        }
      }
      result += m;
    }
    result
  }
}

pub fn run(content: &str) {
  let cards: Vec<Card> = content.lines().map(Card::new).collect();
  let res1 = cards.iter().map(|x| x.score()).sum::<u32>();
  let res2 = Card::roll(&cards);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

  #[test]
  fn small() {
    let scores: Vec<u32> = TEST.lines()
      .map(super::Card::new)
      .map(|x| x.score()).collect();
    assert_eq!(scores, [8, 2, 2, 1, 0, 0]);
  }

  #[test]
  fn large() {
    let cards: Vec<super::Card> = TEST.lines()
      .map(super::Card::new).collect();
    assert_eq!(super::Card::roll(&cards), 30);
  }
}
