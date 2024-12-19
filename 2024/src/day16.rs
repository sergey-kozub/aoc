use std::collections::{BinaryHeap, HashMap, HashSet};

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

    fn paths(&self) -> usize {
        let end = (self.width() - 2, 1);
        let init = (1, self.height() - 2);
        let path = HashSet::from([init]);
        let mut paths = HashMap::from([((init, 0_u8), (path, 0_i32))]);
        let mut queue = BinaryHeap::from([(0_i32, 0_u8, init)]);

        while let Some((score, dir, pos)) = queue.pop() {
            if pos == end { break; }
            let (x, y) = match dir {
                0 => (pos.0 + 1, pos.1),
                1 => (pos.0, pos.1 + 1),
                2 => (pos.0 - 1, pos.1),
                3 => (pos.0, pos.1 - 1),
                _ => panic!(),
            };
            for new_dir in [dir, (dir + 1) % 4, (dir + 3) % 4] {
                let step = new_dir == dir;
                if step && !matches!(self.data[y][x], Cell::Empty) { continue; }
                let new_score = score - if step {1} else {1000};
                let key = if step {((x, y), dir)} else {(pos, new_dir)};
                let mut new_path = paths.get(&(pos, dir)).unwrap().0.clone();
                new_path.insert(key.0);

                match paths.get_mut(&key) {
                    Some(item) => if item.1 == new_score {
                        for t in new_path { item.0.insert(t); }
                    },
                    None => {
                        paths.insert(key, (new_path, new_score));
                        queue.push((new_score, key.1, key.0));
                    },
                }
            }
        }
        HashSet::<(usize, usize)>::from_iter(paths.into_iter()
            .filter(|(k, _)| k.0 == end)
            .flat_map(|(_, v)| v.0.into_iter())
        ).len()
    }
}

pub fn run(content: &str) {
    let grid = Grid::parse(content);
    println!("{} {}", grid.traverse().unwrap(), grid.paths());
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

    #[test]
    fn large() {
        let grid = super::Grid::parse(TEST);
        assert_eq!(grid.paths(), 45);
    }
}
