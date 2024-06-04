use std::collections::HashSet;
use std::iter;

fn parse_num(s: &str) -> i32 {
  s.trim_start_matches('+').parse::<i32>().unwrap()
}

fn reach_twice(a: Vec<&str>) -> Option<i32> {
  let mut visited = HashSet::from([0_i32]);
  let mut sum = 0_i32;
  for s in iter::repeat(a).flatten() {
    sum += parse_num(s);
    if !visited.insert(sum) { return Some(sum); }
  }
  None
}

pub fn run(content: &str) {
  let res1 = content.lines().map(parse_num).sum::<i32>();
  let res2 = reach_twice(content.lines().collect()).unwrap();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn small() {
    let test = |s: &str| s.split(", ").map(super::parse_num).sum::<i32>();
    assert_eq!(test("+1, -2, +3, +1"), 3);
    assert_eq!(test("+1, +1, +1"), 3);
    assert_eq!(test("+1, +1, -2"), 0);
    assert_eq!(test("-1, -2, -3"), -6);
  }

  #[test]
  fn large() {
    let test = |s: &str| super::reach_twice(s.split(", ").collect()).unwrap();
    assert_eq!(test("+1, -1"), 0);
    assert_eq!(test("+3, +3, +4, -2, -4"), 10);
    assert_eq!(test("-6, +3, +8, +5, -6"), 5);
    assert_eq!(test("+7, +7, -2, -7, -4"), 14);
  }
}
