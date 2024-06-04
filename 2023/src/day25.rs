use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Graph {
  names: Vec<String>,
  conn: Vec<Vec<usize>>,
}

impl Graph {
  fn parse(text: &str) -> Graph {
    let mut names: Vec<String> = vec![];
    let mut conn: Vec<Vec<usize>> = vec![];
    let mut rmap = HashMap::<String, usize>::new();
    let mut index = |s: &str| -> usize {
      *rmap.entry(String::from(s)).or_insert_with(|| {
        names.push(String::from(s));
        names.len() - 1
      })
    };
    for line in text.lines() {
      let (s1, s2) = line.split_once(": ").unwrap();
      let i = index(s1);
      if i == conn.len() { conn.push(vec![]); }
      for to in s2.split(' ') {
        let j = index(to);
        if j == conn.len() { conn.push(vec![]); }
        conn[i].push(j);
        conn[j].push(i);
      }
    }
    Graph { names, conn }
  }

  fn find_distinct(&self, from: usize, to: usize, count: usize) -> bool {
    let direct = self.conn[from].contains(&to);
    let mut in_use = HashSet::from([from]);
    'main: for _ in 0..(count - direct as usize) {
      let mut queue = VecDeque::from([vec![from]]);
      let mut visited = in_use.clone();
      while let Some(path) = queue.pop_front() {
        let last = *path.last().unwrap();
        for cur in &self.conn[last] {
          if *cur == to {
            if path.len() == 1 { continue; }
            in_use.extend(path.into_iter());
            continue 'main;
          }
          if !visited.contains(cur) {
            let mut new_path = path.clone();
            new_path.push(*cur);
            queue.push_back(new_path);
            visited.insert(*cur);
          }
        }
      }
      return false;
    }
    true
  }

  fn find_cliques(&self, count: usize) -> Vec<HashSet<usize>> {
    let mut result: Vec<HashSet<usize>> = vec![];
    let mut visited = HashSet::<usize>::new();
    for i in 0..self.names.len() {
      if visited.contains(&i) { continue; }
      let mut connect: Vec<usize> = vec![i];
      for j in i+1..self.names.len() {
        if visited.contains(&j) { continue; }
        if self.find_distinct(i, j, count) { connect.push(j); }
      }
      if connect.len() > 1 {
        visited.extend(connect.iter().cloned());
        result.push(HashSet::from_iter(connect.into_iter()));
      }
    }
    result
  }
}

pub fn run(content: &str) {
  let graph = Graph::parse(content);
  let clique = graph.find_cliques(4);
  let res1 = clique.iter().map(|x| x.len()).product::<usize>();
  println!("{}", res1);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

  #[test]
  fn small() {
    let test = super::Graph::parse(TEST);
    let size = test.find_cliques(4).iter()
      .map(|x| x.len()).collect::<Vec<_>>();
    assert_eq!(size, [6, 9]);
  }
}
