use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
struct Step {
  name: char,
  depends: Vec<usize>,
}

fn from_char(c: char) -> usize { c as usize - 65 }
fn to_char(n: u8) -> char { (n + 65) as char }

impl Step {
  fn parse(text: &str) -> Vec<Step> {
    let mut result = Vec::from_iter((0..26).map(|n| Step {
      name: to_char(n),
      depends: vec![],
    }));
    let mut dmax = 0_usize;
    let number = |s: &str| from_char(s.chars().nth(0).unwrap());
    text.lines().for_each(|s| {
      let a = s.split(' ').collect::<Vec<_>>();
      let (src, dst) = (number(a[1]), number(a[7]));
      result[dst].depends.push(src);
      dmax = cmp::max(dmax, dst + 1);
    });
    result.truncate(dmax);
    result
  }

  fn get_order(data: &[Step]) -> String {
    let mut result: Vec<char> = vec![];
    let mut past = HashSet::<usize>::new();
    while past.len() < data.len() {
      let next = data.iter().filter_map(|x| {
        let n = from_char(x.name);
        if !past.contains(&n) && x.depends.iter().all(|i| past.contains(i))
          {Some(n)} else {None}
      }).min().unwrap();
      past.insert(next);
      result.push(to_char(next as u8));
    }
    result.into_iter().collect::<String>()
  }

  fn get_time(data: &[Step], workers: usize, delay: u32) -> u32 {
    let mut work: Vec<(usize, u32)> = vec![];
    let mut past = HashSet::<usize>::new();
    let mut time = 0_u32;
    while past.len() < data.len() {
      while work.len() < workers {
        let it = data.iter().filter_map(|x| {
          let n = from_char(x.name);
          if !past.contains(&n) && x.depends.iter().all(|i| past.contains(i))
            && !work.iter().any(|t| t.0 == n) {Some(n)} else {None}
        });
        match it.min() {
          Some(i) => work.push((i, time + delay + i as u32 + 1)),
          None => break,
        };
      }
      time = work.iter().map(|x| x.1).min().unwrap();
      let idx = work.iter().position(|x| x.1 == time).unwrap();
      past.insert(work.remove(idx).0);
    }
    time
  }
}

pub fn run(content: &str) {
  let steps = Step::parse(content);
  let res1 = Step::get_order(&steps);
  let res2 = Step::get_time(&steps, 5, 60);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

  #[test]
  fn small() {
    let steps = super::Step::parse(TEST);
    assert_eq!(super::Step::get_order(&steps), "CABDFE");
  }

  #[test]
  fn large() {
    let steps = super::Step::parse(TEST);
    assert_eq!(super::Step::get_time(&steps, 2, 0), 15);
  }
}
