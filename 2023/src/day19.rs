use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug)]
enum Category {
  Extreme,
  Musical,
  Aerodynamic,
  Shiny,
}

#[derive(Clone, Debug)]
enum Action {
  Move(String),
  Accept,
  Reject,
}

#[derive(Clone, Debug)]
struct Condition {
  category: Category,
  compare: Ordering,
  value: u32,
}

#[derive(Clone, Debug)]
struct RuleSet {
  rules: Vec<(Condition, Action)>,
  default: Action,
}

#[derive(Clone, Debug)]
struct Item {
  weight: Vec<u32>,
}

#[derive(Clone, Debug)]
struct ItemRange {
  range: Vec<RangeInclusive<u32>>,
}

#[derive(Debug)]
struct Factory {
  rulesets: HashMap<String, RuleSet>,
  items: Vec<Item>,
}

impl From<char> for Category {
  fn from(ch: char) -> Self {
    match ch {
      'x' => Category::Extreme,
      'm' => Category::Musical,
      'a' => Category::Aerodynamic,
      's' => Category::Shiny,
      _ => panic!("unknown symbol"),
    }
  }
}

impl Action {
  fn parse(text: &str) -> Action {
    match text {
      "A" => Action::Accept,
      "R" => Action::Reject,
      _ => Action::Move(String::from(text)),
    }
  }
}

impl Condition {
  fn parse(text: &str) -> Condition {
    let get = |n: usize| text.chars().nth(n).unwrap();
    let category = Category::from(get(0));
    let compare = match get(1) {
      '>' => Ordering::Greater,
      '<' => Ordering::Less,
      _ => panic!("unknown symbol"),
    };
    let value = text[2..].parse::<u32>().unwrap();
    Condition { category, compare, value }
  }

  fn matches(&self, item: &Item) -> bool {
    item.weight[Item::index(self.category)].cmp(&self.value) == self.compare
  }
}

impl RuleSet {
  fn parse(text: &str) -> RuleSet {
    let a: Vec<&str> = text.split(',').collect();
    let n = a.len() - 1;
    let rules = a[..n].iter().map(|s| {
      let (s1, s2) = s.split_once(':').unwrap();
      (Condition::parse(s1), Action::parse(s2))
    }).collect::<Vec<_>>();
    let default = Action::parse(a[n]);
    RuleSet { rules, default }
  }
}

impl Item {
  fn parse(text: &str) -> Item {
    let mut weight = vec![0_u32; 4];
    text.split(',').for_each(|s| {
      let key = Category::from(s.chars().nth(0).unwrap());
      weight[Item::index(key)] = s[2..].parse::<u32>().unwrap();
    });
    Item { weight }
  }

  fn total(&self) -> u32 {
    self.weight.iter().sum::<u32>()
  }

  fn index(key: Category) -> usize {
    match key {
      Category::Extreme => 0,
      Category::Musical => 1,
      Category::Aerodynamic => 2,
      Category::Shiny => 3,
    }
  }
}

impl ItemRange {
  fn new(init: RangeInclusive<u32>) -> ItemRange {
    let range = vec![init.clone(); 4];
    ItemRange { range }
  }

  fn total(&self) -> u64 {
    self.range.iter().map(|r| {
      (r.end() - r.start() + 1) as u64
    }).product::<u64>()
  }

  fn split(&self, cond: &Condition) -> (Option<ItemRange>, Option<ItemRange>) {
    let idx = Item::index(cond.category);
    let r = &self.range[idx];
    let v = cond.value;
    let c1 = r.start().cmp(&v) == cond.compare;
    let c2 = r.end().cmp(&v) == cond.compare;
    match (c1, c2) {
      (true, true) => (Some(self.clone()), None),
      (false, false) => (None, Some(self.clone())),
      _ => {
        let (r1, r2) = match cond.compare {
          Ordering::Less => (*r.start()..=v-1, v..=*r.end()),
          Ordering::Greater => (v+1..=*r.end(), *r.start()..=v),
          _ => panic!(),
        };
        let (mut x1, mut x2) = (self.clone(), self.clone());
        x1.range[idx] = r1;
        x2.range[idx] = r2;
        (Some(x1), Some(x2))
      },
    }
  }
}

impl Factory {
  fn parse(text: &str) -> Factory {
    let (s1, s2) = text.split_once("\n\n").unwrap();
    let rulesets = s1.lines().map(|s| {
      let (x1, x2) = s.split_once('{').unwrap();
      (String::from(x1), RuleSet::parse(x2.trim_end_matches('}')))
    }).collect::<HashMap<_,_>>();
    let items = s2.lines().map(|s| {
      Item::parse(s.trim_matches(|c| c == '{' || c == '}'))
    }).collect::<Vec<_>>();
    Factory { rulesets, items }
  }

  fn process(&self, item: &Item) -> Action {
    let mut key = String::from("in");
    while let Some(rs) = self.rulesets.get(&key) {
      let action = rs.rules.iter().filter_map(|(k, v)| {
        if k.matches(item) {Some(v)} else {None}
      }).next().unwrap_or(&rs.default);
      match action {
        Action::Move(to) => key = to.clone(),
        _ => return action.clone(),
      }
    }
    panic!("{}", key);
  }

  fn process_all(&self) -> u32 {
    self.items.iter()
      .filter(|item| matches!(self.process(item), Action::Accept))
      .map(|item| item.total()).sum::<u32>()
  }

  fn count(&self, key: String, mut range: ItemRange) -> u64 {
    let mut result = 0_u64;
    let rs = self.rulesets.get(&key).unwrap();
    let update = |a: &Action, r: ItemRange| match a {
      Action::Move(to) => self.count(to.clone(), r),
      Action::Accept => r.total(),
      Action::Reject => 0,
    };
    for (cond, act) in &rs.rules {
      let (a, b) = range.split(cond);
      if let Some(r1) = a {
        result += update(act, r1);
      }
      match b {
        Some(r2) => range = r2,
        None => return result,
      };
    }
    result += update(&rs.default, range);
    result
  }

  fn count_all(&self, init: RangeInclusive<u32>) -> u64 {
    self.count(String::from("in"), ItemRange::new(init))
  }
}

pub fn run(content: &str) {
  let factory = Factory::parse(content);
  let res1 = factory.process_all();
  let res2 = factory.count_all(1..=4000);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

  #[test]
  fn small() {
    let test = super::Factory::parse(TEST);
    assert_eq!(test.process_all(), 19114);
  }

  #[test]
  fn large() {
    let test = super::Factory::parse(TEST);
    assert_eq!(test.count_all(1..=4000), 167409079868000);
  }
}
