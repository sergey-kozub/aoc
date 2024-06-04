use std::collections::HashMap;

#[derive(Debug)]
struct Engine {
  data: Vec<Vec<char>>,
  width: usize,
  height: usize,
}

#[derive(Clone, Debug)]
struct Label {
  x: usize,
  y: usize,
  size: usize,
  value: u32,
}

#[derive(Debug)]
struct Symbol {
  x: usize,
  y: usize,
  value: char,
}

impl Engine {
  fn new(content: &str) -> Engine {
    let data: Vec<Vec<char>> = content.lines()
      .map(|x| x.chars().collect()).collect();
    let width = data[0].len();
    let height = data.len();
    Engine { data, width, height }
  }

  fn labels(&self) -> Vec<Label> {
    let mut result: Vec<Label> = vec![];
    let mut current: Option<Label> = None;
    for y in 0..self.height {
      for x in 0..self.width {
        let digit = self.data[y][x].to_digit(10);
        match current.as_mut() {
          Some(mut label) => match digit {
            Some(v) => {
              label.size += 1;
              label.value = label.value * 10 + v;
            },
            None => result.push(current.take().unwrap()),
          },
          None => current = digit.map(|v| {
            Label { x, y, size: 1, value: v }
          }),
        }
      }
      if let Some(label) = current.take() {
        result.push(label);
      }
    }
    result
  }

  fn adjacent(&self, label: &Label) -> Vec<Symbol> {
    let lb = label.x > 0;
    let rb = label.x + label.size < self.width;
    let left = label.x - lb as usize;
    let right = label.x + label.size - 1 + rb as usize;

    let mut pos: Vec<(usize, usize)> = vec![];
    if label.y > 0 {
      pos.extend((left..=right).map(|x| (x, label.y - 1)));
    }
    if label.y < self.height - 1 {
      pos.extend((left..=right).map(|x| (x, label.y + 1)));
    }
    if lb { pos.push((left, label.y)); }
    if rb { pos.push((right, label.y)); }

    pos.into_iter()
      .map(|(x, y)| Symbol { x, y, value: self.data[y][x] })
      .filter(|t| !t.value.is_ascii_digit() && t.value != '.')
      .collect()
  }

  fn gears(&self, value: char, count: usize) -> Vec<Vec<u32>> {
    let mut items: HashMap<(usize, usize), Vec<Label>> = HashMap::new();
    for label in self.labels() {
      for symbol in self.adjacent(&label) {
        if symbol.value == value {
          items.entry((symbol.x, symbol.y))
            .or_insert(vec![])
            .push(label.clone());
        }
      }
    }
    items.values()
      .filter(|arr| arr.len() == count)
      .map(|arr| arr.iter().map(|t| t.value).collect())
      .collect()
  }
}

pub fn run(content: &str) {
  let engine = Engine::new(content);
  let res1 = engine.labels().iter()
    .filter(|num| engine.adjacent(num).len() > 0)
    .map(|num| num.value).sum::<u32>();
  let res2 = engine.gears('*', 2).iter()
    .map(|a| a[0] * a[1]).sum::<u32>();
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
    467..114..\n\
    ...*......\n\
    ..35..633.\n\
    ......#...\n\
    617*......\n\
    .....+.58.\n\
    ..592.....\n\
    ......755.\n\
    ...$.*....\n\
    .664.598..";

  #[test]
  fn small() {
    let engine = super::Engine::new(TEST);
    let value = engine.labels().iter()
      .filter(|label| engine.adjacent(label).len() > 0)
      .map(|label| label.value).sum::<u32>();
    assert_eq!(value, 4361);
  }

  #[test]
  fn large() {
    let engine = super::Engine::new(TEST);
    let value = engine.gears('*', 2).iter()
      .map(|a| a[0] * a[1]).sum::<u32>();
    assert_eq!(value, 467835);
  }
}
