use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone)]
struct Tile {
    size: usize,
    data: u32,
}

struct Rules {
    small: HashMap<u32, u32>,
    large: HashMap<u32, u32>,
}

struct Field {
    tiles: Vec<Tile>,
    size: usize,
    rules: Rules,
}

impl Tile {
    fn parse(text: &str) -> Self {
        let a = text.split('/').collect::<Vec<_>>();
        let size = a.len();
        let data = a.into_iter().flat_map(|line| line.chars()).enumerate()
            .map(|(k, v)| if v == '#' {1 << k} else {0})
            .sum::<u32>();
        Self { size, data }
    }

    fn is_set(&self, x: usize, y: usize) -> bool {
        let bit = y * self.size + x;
        (self.data & (1 << bit)) != 0
    }

    fn rotate(&self) -> Self {
        let n = self.size;
        let data = (0..n * n).map(|i| {
            let (x, y) = (i % n, i / n);
            if self.is_set(y, n - x - 1) {1 << i} else {0}
        }).sum::<u32>();
        Self { size: n, data }
    }

    fn flip(&self, vertical: bool) -> Self {
        let n = self.size;
        let data = (0..n * n).map(|i| {
            let (mut x, mut y) = (i % n, i / n);
            if vertical {y = n - y - 1} else {x = n - x - 1};
            if self.is_set(x, y) {1 << i} else {0}
        }).sum::<u32>();
        Self { size: n, data }
    }

    fn variants(&self) -> Vec<Self> {
        let mut item = self.clone();
        let unique = (0..4).flat_map(|_| {
            item = item.rotate();
            [item.data, item.flip(false).data, item.flip(true).data]
        }).collect::<HashSet<_>>();
        unique.into_iter().map(|x| Self { size: self.size, data: x }).collect()
    }
}

impl Rules {
    fn parse(text: &str) -> Self {
        let input = text.lines().map(|line| {
            let (l, r) = line.split_once(" => ").unwrap();
            (Tile::parse(l), Tile::parse(r))
        });
        let (mut small, mut large) = (HashMap::new(), HashMap::new());
        for (s, t) in input {
            let dest = if s.size == 2 {&mut small} else {&mut large};
            for v in s.variants().into_iter() {
                assert!(dest.insert(v.data, t.data).is_none());
            }
        }
        Self { small, large }
    }
}

impl Field {
    fn new(rules: Rules) -> Self {
        let init = Tile { size: 3, data: 0x1e2 };
        Self { tiles: vec![init], size: 1, rules }
    }

    fn width(&self) -> usize {
        self.size * self.tiles[0].size
    }

    fn grow<const N: usize>(&mut self) {
        let map = if N == 2 {&self.rules.small} else {&self.rules.large};
        let ext = self.tiles.iter().map(|tile| {
            assert_eq!(tile.size, N);
            Tile { size: N + 1, data: *map.get(&tile.data).unwrap() }
        }).collect::<Vec<_>>();

        let (n, m) = (if N == 2 && self.size % 2 != 0 {3} else {2}, N + 1);
        let (old_size, new_size) = (self.size, (self.width() + self.size) / n);
        self.tiles = (0..new_size * new_size).map(|idx| {
            let (x, y) = (idx % new_size, idx / new_size);
            let sum = (0..n * n).map(|i| {
                let (px, py) = (x * n + i % n, y * n + i / n);
                let tile = &ext[(py / m) * old_size + (px / m)];
                if tile.is_set(px % m, py % m) {1 << i} else {0}
            }).sum::<u32>();
            Tile { size: n, data: sum }
        }).collect::<Vec<_>>();
        self.size = new_size;
    }

    fn advance(&mut self, steps: usize) -> u32 {
        for _ in 1..steps { self.next(); }
        self.next().unwrap()
    }
}

impl Iterator for Field {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.width() % 2 == 0 {
            self.grow::<2>();
        } else {
            self.grow::<3>();
        }
        let it = self.tiles.iter().map(|t| t.data.count_ones());
        Some(it.sum())
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lines = (0..self.size).map(|y| {
            (0..self.size).map(|x| if self.is_set(x, y) {'#'} else {'.'})
                .collect::<String>()
        }).collect::<Vec<_>>();
        write!(f, "{}", lines.join("\n"))
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for y in 0..self.size {
            let data = (0..self.size)
                .map(|x| format!("{}", self.tiles[y * self.size + x]))
                .collect::<Vec<_>>();
            let mut iter = data.iter().map(|s| s.lines()).collect::<Vec<_>>();
            for _ in 0..self.tiles[0].size {
                for it in iter.iter_mut() {
                    result += it.next().unwrap();
                }
                result += "\n";
            }
        }
        write!(f, "{}", result)
    }
}

pub fn run(content: &str) {
    let mut field = Field::new(Rules::parse(content));
    println!("{} {}", field.advance(5), field.advance(13));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        ../.# => ##./#../...\n\
        .#./..#/### => #..#/..../..../#..#";

    #[test]
    fn small() {
        let mut field = super::Field::new(super::Rules::parse(TEST));
        assert_eq!(field.next(), Some(4));
        assert_eq!(field.next(), Some(12));
    }
}
