use std::cmp::Ordering;
use std::fmt;

const CARDS: &str = "*_23456789TJQKA";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Card {
  value: u8,
}

#[derive(Clone, Debug, Eq)]
struct Hand {
  cards: [Card; 5],
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Rank {
  High,
  OnePair,
  TwoPair,
  Three,
  FullHouse,
  Four,
  Five,
}

impl Card {
  fn parse(label: char, joker: bool) -> Card {
    if joker && label == 'J' { return Card { value: 0 }; }
    Card { value: CARDS.find(label).unwrap() as u8 }
  }
}

impl Hand {
  fn parse(labels: &str, joker: bool) -> Hand {
    let cards: Vec<Card> = labels.chars()
      .map(|c| Card::parse(c, joker)).collect();
    Hand { cards: cards.try_into().unwrap() }
  }

  fn parse_game(text: &str, joker: bool) -> Vec<(Hand, u32)> {
    text.lines().map(|line| {
      let (s1, s2) = line.split_once(' ').unwrap();
      (Hand::parse(s1, joker), s2.parse::<u32>().unwrap())
    }).collect()
  }

  fn get_rank(&self) -> Rank {
    let mut count = [0_u8; 15];
    for card in self.cards {
      count[card.value as usize] += 1;
    }
    let wild = count[0] as usize;
    let mut hist = [0_u8; 6];
    for c in count.into_iter().skip(2) {
      hist[c as usize] += 1;
    }
    if hist[5 - wild] != 0 {
      return Rank::Five;
    }
    if hist[4 - wild] != 0 {
      return Rank::Four;
    }
    if hist[3 - wild] != 0 {
      let has_pair = hist[2] >= (if wild == 1 {2} else {1});
      return if has_pair {Rank::FullHouse} else {Rank::Three};
    }
    if hist[2 - wild] != 0 {
      let has_pair = hist[2] >= (if wild == 0 {2} else {1});
      return if has_pair {Rank::TwoPair} else {Rank::OnePair};
    }
    return Rank::High;
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    let a = (self.get_rank(), &self.cards);
    let b = (other.get_rank(), &other.cards);
    a.cmp(&b)
  }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let label = CARDS.as_bytes()[self.value as usize] as char;
    write!(f, "{}", label)
  }
}

impl fmt::Display for Hand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}{}{}{}", self.cards[0], self.cards[1], self.cards[2],
                            self.cards[3], self.cards[4])
  }
}

fn score(mut game: Vec<(Hand, u32)>) -> u32 {
  game.sort();
  game.iter().enumerate().map(|(k, v)| (k + 1) as u32 * v.1).sum()
}

pub fn run(content: &str) {
  let res1 = score(Hand::parse_game(content, false));
  let res2 = score(Hand::parse_game(content, true));
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

  #[test]
  fn small() {
    let game = super::Hand::parse_game(TEST, false);
    assert_eq!(super::score(game), 6440);
  }

  #[test]
  fn large() {
    let game = super::Hand::parse_game(TEST, true);
    assert_eq!(super::score(game), 5905);
  }
}
