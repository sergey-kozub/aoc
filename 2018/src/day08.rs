
#[derive(Debug, Default)]
struct Node {
  children: Vec<Node>,
  metadata: Vec<u32>,
}

impl Node {
  fn parse(text: &str) -> Node {
    let mut it = text.trim_end().split(' ')
      .map(|s| s.parse::<u32>().unwrap());
    let mut nodes = vec![Node::default()];
    let mut queue = vec![(0, 1, 0)];
    while let Some((a, b, c)) = queue.last_mut() {
      if *a < *b {
        *a += 1;
        nodes.push(Node::default());
        queue.push((0_u32, it.next().unwrap(), it.next().unwrap()));
      } else {
        for i in (0..*b).rev() {
          let n = nodes.len() - i as usize - 1;
          let node = nodes.remove(n);
          nodes[n - 1].children.push(node);
        }
        let last = nodes.last_mut().unwrap();
        for _ in 0..*c { last.metadata.push(it.next().unwrap()); }
        queue.pop();
      }
    }
    let mut root = nodes.pop().unwrap();
    root.children.pop().unwrap()
  }

  fn count_meta(&self) -> u32 {
    self.children.iter().map(|x| x.count_meta()).sum::<u32>() +
    self.metadata.iter().sum::<u32>()
  }

  fn count_indexed(&self) -> u32 {
    if self.children.is_empty() {
      self.metadata.iter().sum::<u32>()
    } else {
      self.metadata.iter().map(|&i| {
        if i == 0 || i > self.children.len() as u32 {0}
        else {self.children[i as usize - 1].count_indexed()}
      }).sum::<u32>()
    }
  }
}

pub fn run(content: &str) {
  let root = Node::parse(content);
  let res1 = root.count_meta();
  let res2 = root.count_indexed();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

  #[test]
  fn small() {
    let root = super::Node::parse(TEST);
    assert_eq!(root.count_meta(), 138);
  }

  #[test]
  fn large() {
    let root = super::Node::parse(TEST);
    assert_eq!(root.count_indexed(), 66);
  }
}
