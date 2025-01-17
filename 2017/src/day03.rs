use std::collections::HashMap;

enum Dir { Up, Right, Down, Left }

impl Dir {
    fn next(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
}

struct DirIter {
    pos: (isize, isize),
    dir: Dir,
    steps: usize,
}

impl DirIter {
    fn new() -> Self {
        Self { pos: (-1, 0), dir: Dir::Right, steps: 2 }
    }
}

impl Iterator for DirIter {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            Dir::Up => self.pos.1 += 1,
            Dir::Right => self.pos.0 += 1,
            Dir::Down => self.pos.1 -= 1,
            Dir::Left => self.pos.0 -= 1,
        };
        self.steps -= 1;
        if self.steps == 0 {
            self.dir = self.dir.next();
            self.steps = match self.dir {
                Dir::Up | Dir::Right => self.pos.1.abs() * 2 + 1,
                Dir::Down | Dir::Left => self.pos.0.abs() * 2,
            } as usize;
        }
        Some(self.pos)
    }
}

fn spiral(n: usize) -> usize {
    let p = DirIter::new().skip(n - 1).next().unwrap();
    (p.0.abs() + p.1.abs()) as usize
}

fn spiral_extra(n: usize) -> usize {
    let mut data = HashMap::<(isize, isize), usize>::new();
    for pos in DirIter::new() {
        let mut sum = 0;
        for dx in -1..=1 { for dy in -1..=1 {
            if let Some(v) = data.get(&(pos.0 + dx, pos.1 + dy)) {
                sum += v;
            }
        }}
        data.insert(pos, sum.max(1));
        if sum > n { return sum; }
    }
    panic!();
}

pub fn run(content: &str) {
    let n = content.parse::<usize>().unwrap();
    println!("{} {}", spiral(n), spiral_extra(n));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::spiral(1), 0);
        assert_eq!(super::spiral(12), 3);
        assert_eq!(super::spiral(23), 2);
        assert_eq!(super::spiral(1024), 31);
    }

    #[test]
    fn large() {
        assert_eq!(super::spiral_extra(1), 2);
        assert_eq!(super::spiral_extra(500), 747);
    }
}
