use std::collections::{HashSet, VecDeque};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction { Up, Down, Left, Right }

#[derive(Clone, Debug)]
struct Vortex {
    state: HashSet<(usize, usize, Direction)>,
    size: (usize, usize),
}

impl Vortex {
    fn from(input: &str) -> Vortex {
        let lines: Vec<&str> = input.lines().collect();
        let state = HashSet::from_iter(
            lines.iter().enumerate().flat_map(
                |(y, s)| s.chars().enumerate().filter_map(
                move |(x, c)| match c {
                    '^' => Some((x, y, Direction::Up)),
                    'v' => Some((x, y, Direction::Down)),
                    '<' => Some((x, y, Direction::Left)),
                    '>' => Some((x, y, Direction::Right)),
                    _ => None,
                })
            ));
        let width = lines[0].trim().len() - 2;
        let height = lines.len() - 2;
        Vortex { state, size: (width, height) }
    }

    fn traverse(&mut self, src: (usize, usize), dst: (usize, usize)) -> usize {
        let mut queue = VecDeque::from([(src.0, src.1, 0)]);
        let mut occupied = HashSet::<(usize, usize)>::new();
        let mut visited = HashSet::<(usize, usize, usize)>::new();
        let mut prev = 1;

        while let Some((x, y, step)) = queue.pop_front() {
            if !visited.insert((x, y, step)) {
                continue;
            }
            if step != prev {
                self.state = HashSet::from_iter(self.state.iter().map(|&(x, y, d)| {
                    match d {
                        Direction::Up => (x, if y != 1 {y - 1} else {self.size.1}, d),
                        Direction::Down => (x, if y != self.size.1 {y + 1} else {1}, d),
                        Direction::Left => (if x != 1 {x - 1} else {self.size.0}, y, d),
                        Direction::Right => (if x != self.size.0 {x + 1} else {1}, y, d),
                    }
                }));
                occupied = HashSet::from_iter(self.state.iter().map(|t| (t.0, t.1)));
                prev = step;
            }
            if !occupied.contains(&(x, y)) {
                queue.push_back((x, y, step + 1));
            }
            if (y > 1 && !occupied.contains(&(x, y - 1))) || (x, y) == (1, 1) {
                queue.push_back((x, y - 1, step + 1));
            }
            if (y < self.size.1 && !occupied.contains(&(x, y + 1))) || (x, y) == self.size {
                queue.push_back((x, y + 1, step + 1));
            }
            if x > 1 && !occupied.contains(&(x - 1, y)) && y <= self.size.1 {
                queue.push_back((x - 1, y, step + 1));
            }
            if x < self.size.0 && !occupied.contains(&(x + 1, y)) && y != 0 {
                queue.push_back((x + 1, y, step + 1));
            }
            if (x, y) == dst {
                return step;
            }
        }
        panic!("Path not found")
    }

    fn travel(&mut self, ret: bool) -> usize {
        let end = (self.size.0, self.size.1 + 1);
        let mut result = self.traverse((1, 0), end);
        if ret {
            result += self.traverse(end, (1, 0)) + 1;
            result += self.traverse((1, 0), end) + 1;
        }
        return result
    }
}

impl fmt::Display for Vortex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#.")?;
        for _ in 0..self.size.0 { write!(f, "#")?; }
        writeln!(f)?;
        for y in 1..=self.size.1 {
            write!(f, "#")?;
            for x in 1..=self.size.0 {
                let a = Vec::from_iter(self.state.iter().filter(
                    |&it| { it.0 == x && it.1 == y }));
                write!(f, "{}", if a.len() == 1 {
                    match a[0].2 {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                } else if !a.is_empty() {
                    (48 + a.len()) as u8 as char
                } else {'.'})?;
            }
            writeln!(f, "#")?;
        }
        for _ in 0..self.size.0 { write!(f, "#")?; }
        writeln!(f, ".#")
    }
}

pub fn run(content: &str) {
    let mut inst = Vortex::from(content);
    println!("{} {}", inst.clone().travel(false), inst.travel(true));
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { r#"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
        "#.trim()
    }

    #[test]
    pub fn vortex() {
        let mut inst = super::Vortex::from(example());
        assert_eq!(inst.clone().travel(false), 18);
        assert_eq!(inst.travel(true), 54);
    }
}
