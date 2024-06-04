use std::collections::{HashMap, HashSet, VecDeque};

const START: &str = "broadcaster";

#[derive(Clone, Copy, Debug)]
enum Pulse {
  Low,
  High,
}

#[derive(Clone, Debug)]
enum Module {
  Broadcaster,
  FlipFlop(Pulse),
  Conjunction(HashMap<String, Pulse>),
}

#[derive(Clone, Debug)]
struct Node {
  name: String,
  module: Module,
  target: Vec<String>,
}

#[derive(Clone, Debug)]
struct Relay {
  nodes: HashMap<String, Node>,
}

impl Pulse {
  fn inverse(&self) -> Pulse {
    match self {
      Pulse::Low => Pulse::High,
      Pulse::High => Pulse::Low,
    }
  }
}

impl Module {
  fn process(&mut self, signal: Pulse, from: &str) -> Option<Pulse> {
    match self {
      Module::Broadcaster => Some(signal),
      Module::FlipFlop(state) => {
        match signal {
          Pulse::High => None,
          Pulse::Low => {
            *state = state.inverse();
            Some(*state)
          },
        }
      },
      Module::Conjunction(cmap) => {
        *cmap.get_mut(from).unwrap() = signal;
        let all_high = cmap.values().all(|v| matches!(v, Pulse::High));
        Some(if all_high {Pulse::Low} else {Pulse::High})
      },
    }
  }
}

impl Node {
  fn parse(text: &str) -> Node {
    let (s1, s2) = text.split_once(" -> ").unwrap();
    let target = s2.split(", ").map(String::from).collect::<Vec<_>>();
    let (name, module) = match s1.chars().nth(0).unwrap() {
      '%' => (&s1[1..], Module::FlipFlop(Pulse::Low)),
      '&' => (&s1[1..], Module::Conjunction(HashMap::new())),
      _ => (s1, Module::Broadcaster),
    };
    Node { name: String::from(name), module, target }
  }
}

impl Relay {
  fn parse(text: &str) -> Relay {
    let mut nodes = text.lines().map(Node::parse)
      .map(|x| (x.name.clone(), x)).collect::<HashMap<_,_>>();
    let mut rmap = HashMap::<String, Vec<String>>::new();
    for node in nodes.values() {
      for t in &node.target {
        rmap.entry(t.clone()).or_default().push(node.name.clone());
      }
    }
    for (k, v) in rmap.into_iter() {
      if let Some(x) = nodes.get_mut(&k) {
        if let Module::Conjunction(cmap) = &mut x.module {
          cmap.extend(v.into_iter().map(|s| (s, Pulse::Low)));
        }
      }
    }
    assert!(nodes.contains_key(START));
    Relay { nodes }
  }

  fn press(&mut self, start: &str) -> (usize, usize) {
    let init = (String::new(), String::from(start), Pulse::Low);
    let mut queue = VecDeque::from([init]);
    let (mut low, mut high) = (0_usize, 0_usize);
    while let Some((from, to, signal)) = queue.pop_front() {
      match signal {
        Pulse::Low => low += 1,
        Pulse::High => high += 1,
      };
      if let Some(node) = self.nodes.get_mut(&to) {
        if let Some(next) = node.module.process(signal, &from) {
          queue.extend(node.target.iter()
            .map(|v| (to.clone(), v.clone(), next)));
        }
      }
    }
    (low, high)
  }

  fn repeat(&mut self, count: usize) -> usize {
    let (low, high) = (0..count).fold((0_usize, 0_usize), |acc, _| {
      let cur = self.press(START);
      (acc.0 + cur.0, acc.1 + cur.1)
    });
    low * high
  }

  fn state(&self) -> String {
    self.nodes.values().filter_map(|x| match x.module {
      Module::FlipFlop(v) => Some(if matches!(v, Pulse::High) {'1'} else {'0'}),
      _ => None,
    }).collect()
  }

  fn count_disjoint(&self) -> usize {
    let init = self.nodes.get(START).unwrap();
    init.target.iter().map(|start| {
      let mut partial = self.clone();
      let mut visited = HashSet::<String>::new();
      while visited.insert(partial.state()) {
        partial.press(start);
      }
      visited.len()
    }).product()
  }
}

pub fn run(content: &str) {
  let relay = Relay::parse(content);
  let res1 = relay.clone().repeat(1000);
  let res2 = relay.count_disjoint();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
  const TEST_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

  #[test]
  fn small() {
    let mut t1 = super::Relay::parse(TEST_1);
    let mut t2 = super::Relay::parse(TEST_2);
    assert_eq!(t1.repeat(1000), 32000000);
    assert_eq!(t2.repeat(1000), 11687500);
  }
}
