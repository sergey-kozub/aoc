use std::cmp;

fn fold(text: &str) -> String {
  let mut res: Vec<u8> = vec![];
  for c1 in text.bytes() {
    match res.last() {
      Some(&c0) if (65..91).contains(&cmp::min(c0, c1)) &&
        (c0 as i32 - c1 as i32).abs() == 32 => { res.pop(); },
      _ => res.push(c1),
    }
  }
  String::from_utf8(res).unwrap()
}

fn fold_all(text: &str) -> usize {
  (65_u8..91_u8).map(|n| {
    let mut s = String::from(text);
    s.retain(|c| c != n as char && c != (n + 32) as char);
    fold(&s).len()
  }).min().unwrap()
}

pub fn run(content: &str) {
  let res1 = fold(content.trim_end()).len();
  let res2 = fold_all(content.trim_end());
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn small() {
    assert_eq!(super::fold("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
  }

  #[test]
  fn large() {
    assert_eq!(super::fold_all("dabAcCaCBAcCcaDA"), 4);
  }
}
