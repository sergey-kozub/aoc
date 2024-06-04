use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
struct Claim {
  id: u32,
  pos: (u32, u32),
  size: (u32, u32),
}

impl Claim {
  fn parse(text: &str) -> Claim {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let a: Vec<u32> = re.captures(text).unwrap().iter().skip(1)
      .map(|m| m.unwrap().as_str().parse::<u32>().unwrap()).collect();
    Claim { id: a[0], pos: (a[1], a[2]), size: (a[3], a[4]) }
  }

  fn range_x(&self) -> Range<u32> {
    Range { start: self.pos.0, end: self.pos.0 + self.size.0 }
  }
  fn range_y(&self) -> Range<u32> {
    Range { start: self.pos.1, end: self.pos.1 + self.size.1 }
  }

  fn overlap(claims: &[Claim]) -> usize {
    let mut active = HashMap::<(u32, u32), u32>::new();
    for item in claims {
      item.range_x().for_each(|x| item.range_y().for_each(|y| {
        *active.entry((x, y)).or_insert(0) += 1;
      }));
    }
    active.into_values().filter(|&v| v > 1).count()
  }

  fn intersects(&self, other: &Claim) -> bool {
    let check = |r1: Range<u32>, r2: Range<u32>| -> bool {
      !(r1.start >= r2.end || r2.start >= r1.end)
    };
    check(self.range_x(), other.range_x()) &&
    check(self.range_y(), other.range_y())
  }

  fn find_one(claims: &[Claim]) -> Option<u32> {
    claims.iter().filter_map(|item| {
      let n = claims.iter().filter(|&c| item.intersects(c)).count();
      if n == 1 {Some(item.id)} else {None}
    }).next()
  }
}

pub fn run(content: &str) {
  let claims: Vec<Claim> = content.lines().map(Claim::parse).collect();
  let res1 = Claim::overlap(&claims);
  let res2 = Claim::find_one(&claims).unwrap();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

  #[test]
  fn small() {
    let test = TEST.lines().map(super::Claim::parse).collect::<Vec<_>>();
    assert_eq!(super::Claim::overlap(&test), 4);
  }

  #[test]
  fn large() {
    let test = TEST.lines().map(super::Claim::parse).collect::<Vec<_>>();
    assert_eq!(super::Claim::find_one(&test), Some(3));
  }
}
