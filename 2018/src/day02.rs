use std::collections::HashMap;

fn count(a: &[&str], n: usize) -> usize {
  a.iter().filter(|&s| {
    let mut x = HashMap::<char, usize>::new();
    s.chars().for_each(|c| *x.entry(c).or_insert(0) += 1);
    x.values().any(|t| *t == n)
  }).count()
}

fn find_similar(a: &[&str]) -> Option<String> {
  a.iter().enumerate().flat_map(|(k, v)| (0..k).filter_map(|i| {
    let d: Vec<usize> = a[i].chars().enumerate().zip(v.chars())
      .filter_map(|((j, a), b)| if a != b {Some(j)} else {None}).collect();
    if d.len() == 1 {
      Some(v.chars().enumerate()
        .filter_map(|(i, c)| if i != d[0] {Some(c)} else {None})
        .collect::<String>())
    } else {None}
  })).next()
}

pub fn run(content: &str) {
  let data = content.lines().collect::<Vec<&str>>();
  let res1 = count(&data, 2) * count(&data, 3);
  let res2 = find_similar(&data).unwrap();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn small() {
    let test = vec!["abcdef", "bababc", "abbcde", "abcccd",
                    "aabcdd", "abcdee", "ababab"];
    assert_eq!(super::count(&test, 2), 4);
    assert_eq!(super::count(&test, 3), 3);
  }

  #[test]
  fn large() {
    let test = vec!["abcde", "fghij", "klmno", "pqrst",
                    "fguij", "axcye", "wvxyz"];
    assert_eq!(super::find_similar(&test).unwrap(), "fgij");
  }
}
