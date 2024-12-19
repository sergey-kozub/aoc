use std::collections::{BinaryHeap, HashSet};

#[derive(Clone)]
enum Cell { Empty, Wall }

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<Cell>>,
}

impl Grid {
    fn parse(text: &str) -> Self {
        let data = text.lines().map(|line| {
            line.chars().map(|ch| match ch {
                '.'|'S'|'E' => Cell::Empty,
                '#' => Cell::Wall,
                _ => panic!(),
            }).collect()
        }).collect::<Vec<_>>();
        Self { data }
    }

    fn width(&self) -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }

    fn traverse(&self) -> Option<usize> {
        let end = (self.width() - 2, 1);
        let init = (1, self.height() - 2, 0_u8);
        let mut visited = HashSet::from([init]);
        let mut queue = BinaryHeap::from([(0_i32, init)]);
        while let Some((score, state)) = queue.pop() {
            let mut add = |k: (usize, usize, u8), v: i32| {
                let cell = &self.data[k.1][k.0];
                if matches!(cell, Cell::Empty) && visited.insert(k) {
                    queue.push((v, k));
                }
            };
            let (x, y, dir) = state;
            if (x, y) == end { return Some((-score) as usize); }
            let (nx, ny) = match dir {
                0 => (x + 1, y),
                1 => (x, y + 1),
                2 => (x - 1, y),
                3 => (x, y - 1),
                _ => panic!(),
            };
            add((nx, ny, dir), score - 1);
            add((x, y, (dir + 1) % 4), score - 1000);
            add((x, y, (dir + 3) % 4), score - 1000);
        }
        None
    }
}

pub fn run(content: &str) {
    let grid = Grid::parse(content);
    println!("{}", grid.traverse().unwrap());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        ###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############";

    #[test]
    fn small() {
        let grid = super::Grid::parse(TEST);
        assert_eq!(grid.traverse(), Some(7036));
    }
}
