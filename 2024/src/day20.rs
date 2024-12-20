use std::collections::{HashMap, VecDeque};

type Position = (usize, usize);

#[derive(Debug)]
enum Cell {
    Empty,
    Wall,
    Track(usize, usize),
}

struct Grid {
    data: Vec<Vec<Cell>>,
    start: Position,
    end: Position,
}

impl Grid {
    fn parse(text: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let data = text.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, ch)| match ch {
                'S' => { start = Some((x, y)); Cell::Empty },
                'E' => { end = Some((x, y)); Cell::Empty },
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                _ => panic!(),
            }).collect()
        }).collect();
        Self { data, start: start.unwrap(), end: end.unwrap() }
    }

    fn width(&self) -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }

    fn scan(&self, start: Position) -> HashMap<Position, usize> {
        let mut distance = HashMap::from([(start, 0_usize)]);
        let mut queue = VecDeque::from([start]);
        while let Some((x, y)) = queue.pop_front() {
            let dist = *distance.get(&(x, y)).unwrap() + 1;
            for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                let is_empty = matches!(self.data[ny][nx], Cell::Empty);
                if is_empty && !distance.contains_key(&(nx, ny)) {
                    distance.insert((nx, ny), dist);
                    queue.push_back((nx, ny));
                }
            }
        }
        distance
    }

    fn prepare(&mut self) {
        let d1 = self.scan(self.start);
        let mut d2 = self.scan(self.end);
        for ((x, y), v1) in d1 {
            let v2 = d2.remove(&(x, y)).unwrap();
            self.data[y][x] = Cell::Track(v1, v2);
        }
    }

    fn adjacent(&self, (x, y): Position) -> [Option<usize>; 2] {
        let mut s = if (x, y) == self.start {Some(0)} else {None};
        let mut e = if (x, y) == self.end {Some(0)} else {None};
        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if let Cell::Track(u, v) = &self.data[ny][nx] {
                if s.is_none_or(|t| t > *u) { s = Some(*u + 1); }
                if e.is_none_or(|t| t > *v) { e = Some(*v + 1); }
            }
        }
        [s, e]
    }

    fn track(&self, (x, y): Position) -> Option<(usize, usize)> {
        match &self.data[y][x] {
            Cell::Track(s, e) => Some((*s, *e)),
            _ => None,
        }
    }

    fn find_cheats(&self) -> HashMap<usize, usize> {
        let base = self.track(self.end).unwrap().0;
        let mut count = HashMap::new();
        for y in 1..self.height() - 1 {
            for x in 1..self.width() - 1 {
                if !matches!(self.data[y][x], Cell::Wall) { continue; }
                let [s, e] = self.adjacent((x, y));
                if s.is_some() && e.is_some() {
                    let dist = s.unwrap() + e.unwrap();
                    if dist < base {
                        count.entry(base - dist)
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                }
            }
        }
        count
    }

    fn find_cheats_ext(&self, size: usize) -> HashMap<usize, usize> {
        let base = self.track(self.end).unwrap().0;
        let mut count = HashMap::new();
        for sy in 1..self.height() - 1 {
            let y_min = sy - size.min(sy - 1);
            let y_max = sy + size.min(self.height() - sy - 2);
            for sx in 1..self.width() - 1 {
                if matches!(self.data[sy][sx], Cell::Wall) { continue; }
                let x_min = sx - size.min(sx - 1);
                let x_max = sx + size.min(self.width() - sx - 2);
                let (s1, _) = self.track((sx, sy)).unwrap();
                for ey in y_min..=y_max {
                    for ex in x_min..=x_max {
                        if matches!(self.data[ey][ex], Cell::Wall) { continue; }
                        let dx = sx.max(ex) - sx.min(ex);
                        let dy = sy.max(ey) - sy.min(ey);
                        if dx + dy > size { continue; }
                        let (_, e2) = self.track((ex, ey)).unwrap();
                        let dist = s1 + e2 + dx + dy;
                        if dist < base {
                            count.entry(base - dist)
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                        }
                    }
                }
            }
        }
        count
    }
}

pub fn run(content: &str) {
    let mut grid = Grid::parse(content);
    grid.prepare();
    let v1: usize = grid.find_cheats().into_iter()
        .filter_map(|(k, v)| if k >= 100 {Some(v)} else {None}).sum();
    let v2: usize = grid.find_cheats_ext(20).into_iter()
        .filter_map(|(k, v)| if k >= 100 {Some(v)} else {None}).sum();
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        ###############\n\
        #...#...#.....#\n\
        #.#.#.#.#.###.#\n\
        #S#...#.#.#...#\n\
        #######.#.#.###\n\
        #######.#.#...#\n\
        #######.#.###.#\n\
        ###..E#...#...#\n\
        ###.#######.###\n\
        #...###...#...#\n\
        #.#####.#.###.#\n\
        #.#...#.#.#...#\n\
        #.#.#.#.#.#.###\n\
        #...#...#...###\n\
        ###############";

    #[test]
    fn small() {
        let mut grid = super::Grid::parse(TEST);
        grid.prepare();
        let mut cheats = grid.find_cheats();
        assert_eq!(cheats.remove(&2), Some(14));
        assert_eq!(cheats.remove(&4), Some(14));
        assert_eq!(cheats.remove(&6), Some(2));
        assert_eq!(cheats.remove(&8), Some(4));
        assert_eq!(cheats.remove(&10), Some(2));
        assert_eq!(cheats.remove(&12), Some(3));
        assert_eq!(cheats.remove(&20), Some(1));
        assert_eq!(cheats.remove(&36), Some(1));
        assert_eq!(cheats.remove(&38), Some(1));
        assert_eq!(cheats.remove(&40), Some(1));
        assert_eq!(cheats.remove(&64), Some(1));
        assert!(cheats.is_empty());
    }

    #[test]
    fn large() {
        let mut grid = super::Grid::parse(TEST);
        grid.prepare();
        let mut cheats = grid.find_cheats_ext(20);
        assert_eq!(cheats.remove(&76), Some(3));
        assert_eq!(cheats.remove(&74), Some(4));
        assert_eq!(cheats.remove(&72), Some(22));
        assert_eq!(cheats.remove(&70), Some(12));
    }
}
