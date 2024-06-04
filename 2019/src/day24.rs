use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone)]
struct Field(u64);

impl Field {
    fn from(text: &str) -> Field {
        let value = text.lines().enumerate().flat_map(|(y, line)|
            line.as_bytes().iter().enumerate().map(move |(x, &ch)|
            if ch == b'#' {1_u64 << (y * 5 + x)} else {0})).sum();
        Field(value)
    }

    fn get(&self, n: u8) -> bool {
        self.0 & 1_u64 << n != 0
    }

    fn adjacent(&self, n: u8) -> u8 {
        (n % 5 > 0 && self.get(n - 1)) as u8 +
        (n % 5 < 4 && self.get(n + 1)) as u8 +
        (n >= 5 && self.get(n - 5)) as u8 +
        (n < 20 && self.get(n + 5)) as u8
    }

    fn evolve(&self) -> Field {
        let value = (0..25).map(|i| {
            let (bit, adj) = (self.get(i), self.adjacent(i));
            let rev = if bit {adj != 1} else {adj == 1 || adj == 2};
            if bit ^ rev {1_u64 << i} else {0}
        }).sum();
        Field(value)
    }

    fn repeats(&self) -> Field {
        let mut current = (*self).clone();
        let mut seen: HashSet<u64> = HashSet::new();
        while !seen.contains(&current.0) {
            seen.insert(current.0);
            current = current.evolve();
        }
        current
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = (0..5).map(|y| {
            (0..5).map(|x| if self.get(y * 5 + x) {'#'} else {'.'})
                .collect::<String>()
        }).join("\n");
        write!(f, "{}", result)
    }
}

struct RecursiveField {
    fields: HashMap<i32, Field>,
    min: i32,
    max: i32,
}

impl RecursiveField {
    fn from(text: &str) -> RecursiveField {
        let mut fields = HashMap::new();
        fields.insert(0, Field::from(text));
        RecursiveField { fields, min: 0, max: 0 }
    }

    fn get(&self, level: i32, n: u8) -> bool {
        match self.fields.get(&level) {
            Some(field) => field.get(n),
            None => false,
        }
    }

    fn sum_line(&self, level: i32, line: [u8; 5]) -> u8 {
        line.iter().map(|i| self.get(level, *i) as u8).sum()
    }

    fn adjacent(&self, level: i32, n: u8) -> u8 {
        (match self.fields.get(&level) {
            Some(field) => field.adjacent(n),
            None => 0,
        }) + (match n {
            7 => self.sum_line(level + 1, [0, 1, 2, 3, 4]),
            11 => self.sum_line(level + 1, [0, 5, 10, 15, 20]),
            13 => self.sum_line(level + 1, [4, 9, 14, 19, 24]),
            17 => self.sum_line(level + 1, [20, 21, 22, 23, 24]),
            _ => 0,
        }) +
        (n % 5 == 0 && self.get(level - 1, 11)) as u8 +
        (n % 5 == 4 && self.get(level - 1, 13)) as u8 +
        (n < 5 && self.get(level - 1, 7)) as u8 +
        (n > 19 && self.get(level - 1, 17)) as u8
    }

    fn evolve(&self) -> RecursiveField {
        let mut fields: HashMap<i32, Field> = HashMap::new();
        let (mut min, mut max) = (self.min - 1, self.max + 1);
        for level in min..=max {
            let value = (0..25).map(|i| {
                if i == 12 { return 0; }
                let (bit, adj) = (self.get(level, i), self.adjacent(level, i));
                let rev = if bit {adj != 1} else {adj == 1 || adj == 2};
                if bit ^ rev {1_u64 << i} else {0}
            }).sum();
            if value == 0 && level == min { min += 1; }
            else if value == 0 && level == max { max -= 1; }
            else { fields.insert(level, Field(value)); }
        }
        RecursiveField { fields, min, max }
    }

    fn count(&self) -> usize {
        self.fields.values().flat_map(
            |f| (0..25).map(move |i| f.get(i) as usize)).sum()
    }
}

impl fmt::Debug for RecursiveField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = (self.min..=self.max).map(|key| {
            format!("Depth {}:\n{:?}", key, *self.fields.get(&key).unwrap())
        }).join("\n\n");
        write!(f, "{}", result)
    }
}

pub fn run(content: &str) {
    let field = Field::from(content);
    let mut recursive_field = RecursiveField::from(content);
    for _ in 0..200 { recursive_field = recursive_field.evolve(); }
    println!("{} {}", field.repeats().0, recursive_field.count());
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { "\
        ....#\n\
        #..#.\n\
        #..##\n\
        ..#..\n\
        #...."
    }

    #[test]
    fn part1() {
        let field = super::Field::from(example());
        assert_eq!(2129920, field.repeats().0);
    }

    #[test]
    fn part2() {
        let mut field = super::RecursiveField::from(example());
        for _ in 0..10 { field = field.evolve(); }
        assert_eq!(99, field.count());
    }
}
