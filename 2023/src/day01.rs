use regex::Regex;

fn get_num(value: &str) -> u32 {
  let iter = || value.chars()
    .filter(|c| c.is_ascii_digit())
    .map(|c| c.to_digit(10).unwrap());
  let first = iter().next().unwrap();
  let last = iter().rev().next().unwrap();
  first * 10 + last
}

fn search(value: &str, digits: [&str; 9]) -> u32 {
  let pattern = format!("(\\d|{})", digits.join("|"));
  let re = Regex::new(&pattern).unwrap();
  let mut it = re.find_iter(value).map(|m| m.as_str()).map(|s| match s.len() {
    1 => s.parse::<u32>().unwrap(),
    _ => digits.iter().position(|&v| v == s).unwrap() as u32 + 1,
  });
  it.next().unwrap()
}

fn get_str(value: &str) -> u32 {
  let digits =
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  let digits_rev =
    ["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
  let value_rev = value.chars().rev().collect::<String>();
  search(value, digits) * 10 + search(&value_rev, digits_rev)
}

pub fn run(content: &str) {
  let res1 = content.lines().map(get_num).sum::<u32>();
  let res2 = content.lines().map(get_str).sum::<u32>();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn small() {
    assert_eq!(super::get_num("1abc2"), 12);
    assert_eq!(super::get_num("pqr3stu8vwx"), 38);
    assert_eq!(super::get_num("a1b2c3d4e5f"), 15);
    assert_eq!(super::get_num("treb7uchet"), 77);
  }

  #[test]
  fn large() {
    assert_eq!(super::get_str("two1nine"), 29);
    assert_eq!(super::get_str("eightwothree"), 83);
    assert_eq!(super::get_str("abcone2threexyz"), 13);
    assert_eq!(super::get_str("xtwone3four"), 24);
    assert_eq!(super::get_str("4nineeightseven2"), 42);
    assert_eq!(super::get_str("zoneight234"), 14);
    assert_eq!(super::get_str("7pqrstsixteen"), 76);
  }
}
