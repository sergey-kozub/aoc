use std::collections::HashMap;
use std::fmt;

type Point = (usize, usize);
type CartState = (Point, Cart, Turn, bool);

#[derive(Clone, Copy, Debug)]
enum Track {
  Horizontal,
  Vertical,
  Intersection,
  TurnSE,
  TurnSW,
  TurnNE,
  TurnNW,
}

#[derive(Clone, Copy, Debug)]
enum Cart {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone, Copy, Debug)]
enum Turn {
  Left,
  Straight,
  Right,
}

#[derive(Clone)]
struct Field {
  tracks: HashMap<Point, Track>,
  carts: Vec<CartState>,
}

impl Cart {
  fn next(&self, turn: Turn) -> Cart {
    match turn {
      Turn::Straight => *self,
      Turn::Left => self.left(),
      Turn::Right => self.right(),
    }
  }

  fn right(&self) -> Cart {
    match self {
      Cart::Up => Cart::Right,
      Cart::Down => Cart::Left,
      Cart::Left => Cart::Up,
      Cart::Right => Cart::Down,
    }
  }

  fn left(&self) -> Cart {
    self.right().right().right()
  }

  fn vertical(&self) -> bool {
    matches!(self, Cart::Up | Cart::Down)
  }
}

impl Turn {
  fn next(&self) -> Turn {
    match self {
      Turn::Left => Turn::Straight,
      Turn::Straight => Turn::Right,
      Turn::Right => Turn::Left,
    }
  }
}

impl Field {
  fn parse(text: &str) -> Field {
    let mut tracks = HashMap::<Point, Track>::new();
    let mut carts: Vec<CartState> = vec![];
    let not_vertical = |t: &Track| !matches!(t, Track::Vertical);
    let not_horizontal = |t: &Track| !matches!(t, Track::Horizontal);
    for (y, s) in text.lines().enumerate() {
      for (x, c) in s.chars().enumerate() {
        let mut add = |cart: Cart| carts.push(((x, y), cart, Turn::Left, true));
        if let Some(t) = match c {
          '-' => Some(Track::Horizontal),
          '|' => Some(Track::Vertical),
          '+' => Some(Track::Intersection),
          '\\' => Some(Track::TurnSW),
          '/' => {
            if x != 0 && y != 0 &&
              tracks.get(&(x - 1, y)).map_or(false, not_vertical) &&
              tracks.get(&(x, y - 1)).map_or(false, not_horizontal)
            {Some(Track::TurnNW)} else {Some(Track::TurnSE)}
          },
          '^' => { add(Cart::Up); Some(Track::Vertical) },
          'v' => { add(Cart::Down); Some(Track::Vertical) },
          '<' => { add(Cart::Left); Some(Track::Horizontal) },
          '>' => { add(Cart::Right); Some(Track::Horizontal) },
          ' ' => None,
          _ => panic!("unknown symbol"),
        } { tracks.insert((x, y), t); }
      }
    }
    let copy = tracks.clone();
    for (&(x, y), v) in tracks.iter_mut() {
      if matches!(v, Track::TurnSW) && y != 0 &&
        copy.get(&(x + 1, y)).map_or(false, not_vertical) &&
        copy.get(&(x, y - 1)).map_or(false, not_horizontal)
        { *v = Track::TurnNE; }
    }
    Field { tracks, carts }
  }

  fn advance(&self, ((x, y), cart, turn, active): CartState) -> CartState {
    assert!(active);
    let pos = match cart {
      Cart::Up => (x, y - 1),
      Cart::Down => (x, y + 1),
      Cart::Left => (x - 1, y),
      Cart::Right => (x + 1, y),
    };
    let mut nt = turn;
    let nc = match self.tracks.get(&pos).unwrap() {
      Track::Horizontal | Track::Vertical => cart,
      Track::Intersection => { nt = turn.next(); cart.next(turn) },
      Track::TurnSE => if cart.vertical() {Cart::Right} else {Cart::Down},
      Track::TurnSW => if cart.vertical() {Cart::Left} else {Cart::Down},
      Track::TurnNE => if cart.vertical() {Cart::Right} else {Cart::Up},
      Track::TurnNW => if cart.vertical() {Cart::Left} else {Cart::Up},
    };
    (pos, nc, nt, true)
  }

  fn advance_all(&self) -> Vec<CartState> {
    let mut pos = HashMap::<Point, usize>::new();
    for state in &self.carts {
      *pos.entry(state.0).or_default() += 1;
    }
    let mut new = self.carts.iter().map(|&(p, c, t, a)| {
      let mut count = pos.get_mut(&p).unwrap();
      if *count > 1 { return (p, c, t, false); }
      *count -= 1;
      let next = self.advance((p, c, t, a));
      count = pos.entry(next.0).or_default();
      *count += 1;
      next
    }).collect::<Vec<_>>();
    for t in new.iter_mut() {
      if *pos.get(&t.0).unwrap_or(&0) > 1 { t.3 = false; }
    }
    new.sort_by_key(|t| t.0);
    new
  }

  fn step(&mut self) -> usize {
    self.carts = self.advance_all();
    self.carts.iter().filter(|t| t.3).count()
  }

  fn first_crash(&mut self) -> Point {
    while self.step() == self.carts.len() {}
    self.carts.iter().filter_map(|t| {
      if !t.3 {Some(t.0)} else {None}
    }).next().unwrap()
  }

  fn last_alive(&mut self) -> Point {
    while self.carts.len() > 1 {
      while self.step() == self.carts.len() {}
      self.carts.retain(|t| t.3);
    }
    self.carts[0].0
  }
}

impl fmt::Debug for Field {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let x_max = self.tracks.keys().map(|k| k.0).max().unwrap();
    let y_max = self.tracks.keys().map(|k| k.1).max().unwrap();
    let mut carts = HashMap::<Point,Vec<Cart>>::new();
    for (p, c, _, _) in &self.carts {
      carts.entry(*p).or_default().push(*c);
    }
    for y in 0..=y_max {
      let line = (0..=x_max).map(|x| {
        if let Some(c) = carts.get(&(x, y)) {
          if c.len() > 1 { return 'X'; }
          return match c[0] {
            Cart::Up => '^',
            Cart::Down => 'v',
            Cart::Left => '<',
            Cart::Right => '>',
          }
        }
        if let Some(t) = self.tracks.get(&(x, y)) {
          return match t {
            Track::Horizontal => '-',
            Track::Vertical => '|',
            Track::Intersection => '+',
            Track::TurnSE | Track::TurnNW => '/',
            Track::TurnSW | Track::TurnNE => '\\',
          }
        }
        ' '
      }).collect::<String>();
      writeln!(f, "{}", line)?;
    }
    Ok(())
  }
}

pub fn run(content: &str) {
  let mut field = Field::parse(content);
  let res1 = field.clone().first_crash();
  let res2 = field.last_alive();
  println!("{:?} {:?}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = r"
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/";
  const TEST_2: &str = r"
/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

  #[test]
  fn small() {
    let mut test = super::Field::parse(TEST_1.trim());
    assert_eq!(test.first_crash(), (7, 3));
  }

  #[test]
  fn large() {
    let mut test = super::Field::parse(TEST_2.trim());
    assert_eq!(test.last_alive(), (6, 4));
  }
}
