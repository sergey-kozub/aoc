use std::cmp;

#[derive(Debug)]
struct ARange {
  from: u64,
  to: u64,
  size: u64,
}

#[derive(Debug)]
struct AMap {
  _name: String,
  ranges: Vec<ARange>,
}

#[derive(Debug)]
struct Almanac {
  seeds: Vec<u64>,
  maps: Vec<AMap>,
}

impl ARange {
  fn left(&self) -> u64 { self.from }
  fn right(&self) -> u64 { self.from + self.size - 1 }
  fn convert(&self, value: u64) -> u64 { self.to + value - self.from }
  fn intersects(&self, range: (u64, u64)) -> bool {
    !(range.0 > self.right() || range.1 < self.left())
  }
}

impl AMap {
  fn map(&self, source: u64) -> u64 {
    self.ranges.iter().filter(|range| {
      source >= range.left() && source <= range.right()
    }).next().map_or(source, |range| range.convert(source))
  }

  fn rmap(&self, source: (u64, u64)) -> Vec<(u64, u64)> {
    let mut results = Vec::<(u64, u64)>::new();
    let mut parts = vec![source];
    for range in &self.ranges {
      parts = parts.into_iter().flat_map(|src| {
        if range.intersects(src) {
          let l = cmp::max(src.0, range.left());
          let r = cmp::min(src.1, range.right());
          results.push((range.convert(l), range.convert(r)));
          let mut rest = vec![];
          if l != src.0 { rest.push((src.0, l - 1)); }
          if r != src.1 { rest.push((r + 1, src.1)); }
          rest
        } else { vec![src] }
      }).collect();
    }
    results.extend(parts);
    results
  }
}

impl Almanac {
  fn parse(text: &str) -> Almanac {
    let mut parts = text.split("\n\n");
    let (_, s1) = parts.next().unwrap().split_once(' ').unwrap();
    let seeds = s1.split(' ').map(|x| x.parse::<u64>().unwrap());
    let maps = parts.map(|part| {
      let mut lines = part.lines();
      let (s2, _) = lines.next().unwrap().split_once(' ').unwrap();
      let ranges = lines.map(|x| {
        let a: Vec<u64> = x.split(' ')
          .map(|x| x.parse::<u64>().unwrap()).collect();
        ARange { from: a[1], to: a[0], size: a[2] }
      }).collect();
      AMap { _name: s2.into(), ranges }
    });
    Almanac {
      seeds: seeds.collect(),
      maps: maps.collect(),
    }
  }

  fn map(&self, seed: u64) -> u64 {
    self.maps.iter().fold(seed, |v, m| m.map(v))
  }

  fn map_all(&self) -> Vec<u64> {
    self.seeds.iter().map(|&x| self.map(x)).collect()
  }

  fn rmap(&self, seed_range: (u64, u64)) -> Vec<(u64, u64)> {
    self.maps.iter().fold(vec![seed_range], |a, m| {
      a.iter().flat_map(|&v| m.rmap(v)).collect()
    })
  }

  fn rmap_all(&self) -> Vec<u64> {
    let get_range = |p, n| (p, p + n - 1);
    let get_from = |i| get_range(self.seeds[i], self.seeds[i + 1]);
    (0..self.seeds.len()).step_by(2)
      .map(|i| self.rmap(get_from(i)))
      .map(|x| x.iter().min().unwrap().0)
      .collect()
  }
}

pub fn run(content: &str) {
  let almanac = Almanac::parse(content);
  let res1 = almanac.map_all().into_iter().min().unwrap();
  let res2 = almanac.rmap_all().into_iter().min().unwrap();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

  #[test]
  fn small() {
    let almanac = super::Almanac::parse(TEST);
    assert_eq!(almanac.map_all(), [82, 43, 86, 35]);
  }

  #[test]
  fn large() {
    let almanac = super::Almanac::parse(TEST);
    assert_eq!(almanac.rmap_all(), [46, 56]);
  }
}
