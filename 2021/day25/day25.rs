use std::collections::HashSet;
use std::fmt;
use std::fs;

type Position = (i32, i32);

#[derive(Clone, PartialEq)]
struct Seabed {
    horizontal: HashSet<Position>,
    vertical: HashSet<Position>,
    width: usize,
    height: usize,
}

impl Seabed {
    fn from(text: &str) -> Seabed {
        let lines: Vec<&[u8]> = text.lines().map(|s| s.as_bytes()).collect();
        let mut horizontal: HashSet<Position> = HashSet::new();
        let mut vertical: HashSet<Position> = HashSet::new();
        let width = lines[0].len();
        let height = lines.len();

        for i in 0..height {
            for j in 0..width {
                let p = (j as i32, i as i32);
                match lines[i][j] {
                    b'>' => horizontal.insert(p),
                    b'v' => vertical.insert(p),
                    _ => false,
                };
            }
        }
        Seabed { horizontal, vertical, width, height }
    }

    fn next(&self) -> Seabed {
        let mut horizontal: HashSet<Position> = HashSet::new();
        let mut vertical: HashSet<Position> = HashSet::new();

        for p in &self.horizontal {
            let to = ((p.0 + 1) % self.width as i32, p.1);
            horizontal.insert(
                if !self.horizontal.contains(&to) && !self.vertical.contains(&to)
                {to} else {*p});
        }
        for p in &self.vertical {
            let to = (p.0, (p.1 + 1) % self.height as i32);
            vertical.insert(
                if !horizontal.contains(&to) && !self.vertical.contains(&to)
                {to} else {*p});
        }
        Seabed { horizontal, vertical, ..*self }
    }

    fn next_all(&self) -> (Seabed, usize) {
        let mut current = (*self).clone();
        let mut steps = 0;
        loop {
            steps += 1;
            let next = current.next();
            if next == current { break; }
            current = next;
        }
        (current, steps)
    }
}

impl fmt::Debug for Seabed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = (0..self.height as i32).map(|i| {
            (0..self.width as i32).map(|j| {
                if self.horizontal.contains(&(j, i)) {'>'}
                else if self.vertical.contains(&(j, i)) {'v'}
                else {'.'}
            }).collect::<String>()
        }).fold(String::new(), |a, b| a + &b + "\n");
        write!(f, "{}", result)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input");
    let result = Seabed::from(&input).next_all();
    println!("{}", result.1);
}
