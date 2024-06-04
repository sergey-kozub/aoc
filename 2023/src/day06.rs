use std::iter;

#[derive(Debug)]
struct Race {
  time: u64,
  distance: u64,
}

impl Race {
  fn parse(text: &str) -> Vec<Race> {
    let (s1, s2) = text.split_once("\n").unwrap();
    let parse_list = |s: &str| -> Vec<u64> {
      s.split_whitespace().skip(1)
        .map(|x| x.parse::<u64>().unwrap()).collect()
    };
    iter::zip(parse_list(s1), parse_list(s2))
      .map(|(t, d)| Race { time: t, distance: d }).collect()
  }

  fn parse_one(text: &str) -> Race {
    let (s1, s2) = text.split_once("\n").unwrap();
    let parse_num = |s: &str| -> u64 {
      s.split_whitespace().skip(1).collect::<Vec<&str>>()
        .join("").parse::<u64>().unwrap()
    };
    Race { time: parse_num(s1), distance: parse_num(s2) }
  }

  fn count_winning(&self) -> u64 {
    (1..self.time).filter(|t|
      (self.time - t) * t > self.distance
    ).count() as u64
  }
}

pub fn run(content: &str) {
  let races = Race::parse(content);
  let res1: u64 = races.iter().map(|x| x.count_winning()).product();
  let res2 = Race::parse_one(content).count_winning();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
    Time:      7  15   30\n\
    Distance:  9  40  200";

  #[test]
  fn small() {
    let races = super::Race::parse(TEST);
    let winning: Vec<u64> = races.iter().map(|x| x.count_winning()).collect();
    assert_eq!(winning, [4, 8, 9]);
  }

  #[test]
  fn large() {
    let race = super::Race::parse_one(TEST);
    assert_eq!(race.count_winning(), 71503);
  }
}
