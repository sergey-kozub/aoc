use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

#[derive(Clone, Debug)]
enum Cell {
    Empty,
    Wall,
    POI(u8),
}

#[derive(Clone, Debug)]
struct Maze {
    cells: Vec<Vec<Cell>>,
    poi: Vec<Point>,
}

impl Maze {
    fn parse(text: &str) -> Self {
        let cells = text.lines().map(|line| {
            line.chars().map(|c| match c {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                '0'..='9' => Cell::POI(c as u8 - '0' as u8),
                _ => panic!(),
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let mut poi = cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if let Cell::POI(n) = cell {Some((n, x, y))} else {None}
            })
        }).collect::<Vec<_>>();
        poi.sort();
        let poi = poi.into_iter().map(|t| (t.1, t.2)).collect::<Vec<_>>();
        Self { cells, poi }
    }

    fn adjacent(&self, (x, y): Point) -> Vec<Point> {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
            .filter_map(|(x, y)| match self.cells[y][x] {
                Cell::Wall => None,
                _ => Some((x, y)),
            }).collect::<Vec<_>>()
    }

    fn collect(&self, ret: bool) -> usize {
        let (x, y) = self.poi[0];
        let mut queue = VecDeque::from([(0, 1, x, y)]);
        let mut visited = HashSet::from([(1, x, y)]);
        let target = (1 << self.poi.len()) - 1;
        while let Some((steps, mut mask, x, y)) = queue.pop_front() {
            if let Cell::POI(n) = self.cells[y][x] {
                mask |= 1 << n as u32;
                if mask == target {
                    if !ret || n == 0 { return steps; }
                }
            }
            for (x, y) in self.adjacent((x, y)) {
                if visited.insert((mask, x, y)) {
                    queue.push_back((steps + 1, mask, x, y));
                }
            }
        }
        usize::MAX
    }
}

pub fn run(content: &str) {
    let maze = Maze::parse(content);
    let res1 = maze.collect(false);
    let res2 = maze.collect(true);
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn small() {
        let maze = super::Maze::parse(TEST);
        assert_eq!(maze.collect(false), 14);
    }
}
