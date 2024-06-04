
#[derive(Debug)]
enum Operation {
  Add(String, u8),
  Remove(String),
}

#[derive(Debug)]
struct HashState {
  state: [Vec<(String, u8)>; 256],
}

impl Operation {
  fn parse(text: &str) -> Operation {
    if text.ends_with('-') {
      return Operation::Remove(String::from(text.trim_end_matches('-')));
    }
    let (s1, s2) = text.split_once('=').unwrap();
    Operation::Add(String::from(s1), s2.parse::<u8>().unwrap())
  }

  fn index(&self) -> usize {
    make_hash(match self {
      Operation::Add(s, _) => s,
      Operation::Remove(s) => s,
    }) as usize
  }
}

impl HashState {
  fn new() -> HashState {
    HashState { state: std::array::from_fn(|_| vec![]) }
  }

  fn process(&mut self, op: Operation) {
    let bucket = &mut self.state[op.index()];
    match op {
      Operation::Add(s, n) => match bucket.iter().position(|x| x.0 == s) {
        Some(i) => bucket[i].1 = n,
        None => bucket.push((s, n)),
      },
      Operation::Remove(s) => bucket.retain(|x| x.0 != s),
    }
  }

  fn process_all(&mut self, ops: Vec<Operation>) {
    ops.into_iter().for_each(|op| self.process(op));
  }

  fn power(&self) -> usize {
    self.state.iter().enumerate().map(
      |(i, a)| (i + 1) * a.iter().enumerate().map(
      |(j, b)| (j + 1) * b.1 as usize).sum::<usize>()
    ).sum()
  }
}

fn make_hash(text: &str) -> u32 {
  text.chars().fold(0_u32, |a, b| {
    (a + b as u32) * 17 % 256
  })
}

pub fn run(content: &str) {
  let raw: Vec<&str> = content.trim_end().split(',').collect();
  let mut state = HashState::new();
  state.process_all(raw.iter().map(|&s| Operation::parse(s)).collect());
  let res1 = raw.iter().map(|&s| make_hash(s)).sum::<u32>();
  let res2 = state.power();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

  #[test]
  fn small() {
    let hash = TEST.split(',').map(super::make_hash).collect::<Vec<u32>>();
    assert_eq!(hash, [30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);
  }

  #[test]
  fn large() {
    let mut state = super::HashState::new();
    state.process_all(TEST.split(',').map(super::Operation::parse).collect());
    assert_eq!(state.power(), 145);
  }
}
