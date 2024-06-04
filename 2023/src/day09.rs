
fn parse_all(text: &str) -> Vec<Vec<i32>> {
  text.lines().map(|line| {
    line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect()
  }).collect()
}

fn predict(data: &[i32], last: bool) -> i32 {
  if data.iter().all(|&x| x == 0) { return 0; }
  let roll: Vec<i32> = data.windows(2).map(|s| s[1] - s[0]).collect();
  let pred = predict(&roll, last);
  if last {*data.last().unwrap() + pred} else {data[0] - pred}
}

pub fn run(content: &str) {
  let items = parse_all(content);
  let res1: i64 = items.iter().map(|x| predict(x, true) as i64).sum();
  let res2: i64 = items.iter().map(|x| predict(x, false) as i64).sum();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

  #[test]
  fn small() {
    let pred: Vec<i32> = super::parse_all(TEST).iter()
      .map(|x| super::predict(x, true)).collect();
    assert_eq!(pred, [18, 28, 68]);
  }

  #[test]
  fn large() {
    let pred: Vec<i32> = super::parse_all(TEST).iter()
      .map(|x| super::predict(x, false)).collect();
    assert_eq!(pred, [-3, 0, 5]);
  }
}
