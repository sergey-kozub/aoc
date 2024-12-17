use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, Debug)]
enum Move { U, R, D, L }

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Box,
}

struct Grid {
    data: Vec<Vec<Cell>>,
    robot: (usize, usize),
}

#[derive(Clone, Copy, PartialEq)]
enum WideCell {
    Empty,
    Wall,
    Left,
    Right,
}

struct WideGrid {
    data: Vec<Vec<WideCell>>,
    robot: (usize, usize),
}

impl Move {
    fn parse(text: &str) -> Vec<Self> {
        text.chars().map(|c| match c {
            '^' => Self::U,
            '>' => Self::R,
            'v' => Self::D,
            '<' => Self::L,
            _ => panic!(),
        }).collect()
    }
}

impl Grid {
    fn parse(text: &str) -> Self {
        let mut robot: Option<(usize, usize)> = None;
        let data = text.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, ch)| match ch {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                'O' => Cell::Box,
                '@' => { robot = Some((x, y)); Cell::Empty },
                _ => panic!(),
            }).collect()
        }).collect();
        Self { data, robot: robot.unwrap() }
    }

    fn width(&self) -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }

    fn step(&mut self, dir: Move) {
        let (dx, dy) = match dir {
            Move::U => (0, -1),
            Move::R => (1, 0),
            Move::D => (0, 1),
            Move::L => (-1, 0),
        };
        let sx = (self.robot.0 as isize + dx) as usize;
        let sy = (self.robot.1 as isize + dy) as usize;
        let (mut tx, mut ty) = (sx as isize, sy as isize);
        loop {
            match self.data[ty as usize][tx as usize] {
                Cell::Box => (tx, ty) = (tx + dx, ty + dy),
                Cell::Wall => break,
                Cell::Empty => {
                    self.data[ty as usize][tx as usize] = Cell::Box;
                    self.data[sy][sx] = Cell::Empty;
                    self.robot = (sx, sy);
                    break;
                },
            }
        }
    }

    fn simulate(&mut self, steps: &[Move]) -> usize {
        for i in steps {
            self.step(*i);
        }
        (0..self.height()).flat_map(|y| {
            let data = &self.data;
            (0..self.width()).filter_map(move |x| {
                if data[y][x] == Cell::Box {Some(y * 100 + x)} else {None}
            })
        }).sum::<usize>()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            let line = (0..self.width()).map(|x| {
                if (x, y) == self.robot { return '@'; }
                match &self.data[y][x] {
                    Cell::Empty => '.',
                    Cell::Wall => '#',
                    Cell::Box => 'O',
                }
            }).collect::<String>();
            writeln!(f, "{}\r", line)?;
        }
        Ok(())
    }
}

impl WideGrid {
    fn from(grid: &Grid) -> Self {
        let data = grid.data.iter().map(|row| {
            row.iter().flat_map(|cell| match cell {
                Cell::Empty => [WideCell::Empty; 2],
                Cell::Wall => [WideCell::Wall; 2],
                Cell::Box => [WideCell::Left, WideCell::Right],
            }).collect()
        }).collect();
        let (x, y) = grid.robot;
        Self { data, robot: (x * 2, y) }
    }

    fn width(&self) -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }

    fn step(&mut self, dir: Move) {
        let add = |a: usize, b: isize| (a as isize + b) as usize;

        // Simple case: empty or wall.
        let (dx, dy) = match dir {
            Move::U => (0, -1),
            Move::R => (1, 0),
            Move::D => (0, 1),
            Move::L => (-1, 0),
        };
        let (rx, ry) = (add(self.robot.0, dx), add(self.robot.1, dy));
        match &self.data[ry][rx] {
            WideCell::Empty => { self.robot = (rx, ry); return; },
            WideCell::Wall => return,
            _ => {},
        };

        // Collect list of boxes and sort.
        let mut boxes = self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if matches!(cell, WideCell::Left) {Some((x, y))} else {None}
            })
        }).collect::<Vec<_>>();
        match dir {
            Move::U => boxes.sort_by_key(|t| t.1),
            Move::R => boxes.sort_by_key(|t| t.0 as isize * -1),
            Move::D => boxes.sort_by_key(|t| t.1 as isize * -1),
            Move::L => boxes.sort_by_key(|t| t.0),
        };

        // Determine which boxes can be moved.
        let mut movable = HashSet::new();
        let right = if matches!(dir, Move::R) {1} else {0};
        for (x, y) in boxes {
            let (px, py) = (add(x, dx) + right, add(y, dy));
            let can_move = (0..if dx == 0 {2} else {1}).all(|i| {
                match &self.data[py][px + i] {
                    WideCell::Empty => true,
                    WideCell::Wall => false,
                    WideCell::Left => movable.contains(&(px + i, py)),
                    WideCell::Right => movable.contains(&(px + i - 1, py)),
                }
            });
            if can_move {
                movable.insert((x, y));
            }
        }

        // Determine which boxes to move.
        let mut moving = HashSet::new();
        let mut queue = vec![(rx, ry)];
        while let Some((x, y)) = queue.pop() {
            let (px, py) = match &self.data[y][x] {
                WideCell::Empty => continue,
                WideCell::Wall => panic!(),
                WideCell::Left => (x, y),
                WideCell::Right => (x - 1, y),
            };
            let ok = movable.contains(&(px, py));
            if !ok && moving.is_empty() { return; }
            assert!(ok);
            moving.insert((px, py));
            for i in 0..if dx == 0 {2} else {1} {
                let (nx, ny) = (add(px, dx) + right, add(py, dy));
                queue.push((nx + i, ny));
            }
        }

        // Move the boxes.
        for &(x, y) in &moving {
            self.data[y][x] = WideCell::Empty;
            self.data[y][x + 1] = WideCell::Empty;
        }
        for &(x, y) in &moving {
            let (nx, ny) = (add(x, dx), add(y, dy));
            self.data[ny][nx] = WideCell::Left;
            self.data[ny][nx + 1] = WideCell::Right;
        }
        self.robot = (rx, ry);
    }

    fn simulate(&mut self, steps: &[Move]) -> usize {
        for i in steps {
            self.step(*i);
        }
        (0..self.height()).flat_map(|y| {
            let data = &self.data;
            (0..self.width()).filter_map(move |x| {
                if data[y][x] == WideCell::Left {Some(y * 100 + x)} else {None}
            })
        }).sum::<usize>()
    }
}

impl fmt::Display for WideGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            let line = (0..self.width()).map(|x| {
                if (x, y) == self.robot { return '@'; }
                match &self.data[y][x] {
                    WideCell::Empty => '.',
                    WideCell::Wall => '#',
                    WideCell::Left => '[',
                    WideCell::Right => ']',
                }
            }).collect::<String>();
            writeln!(f, "{}\r", line)?;
        }
        Ok(())
    }
}

fn parse(text: &str) -> (Grid, Vec<Move>) {
    let (l, r) = text.split_once("\n\n").unwrap();
    let s = r.replace("\n", "");
    (Grid::parse(l), Move::parse(&s))
}

pub fn run(content: &str) {
    let (mut grid, moves) = parse(content);
    let mut wide = WideGrid::from(&grid);
    println!("{} {}", grid.simulate(&moves), wide.simulate(&moves));
}

#[cfg(test)]
mod tests {
    const TEST_1: &str = "\
        ########\n\
        #..O.O.#\n\
        ##@.O..#\n\
        #...O..#\n\
        #.#.O..#\n\
        #...O..#\n\
        #......#\n\
        ########\n\
                \n\
        <^^>>>vv<v>>v<<";
    const TEST_2: &str = "\
        ##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
                  \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn small() {
        let score = |s| { let (mut g, m) = super::parse(s); g.simulate(&m) };
        assert_eq!(score(TEST_1), 2028);
        assert_eq!(score(TEST_2), 10092);
    }

    #[test]
    fn large() {
        let (grid, moves) = super::parse(TEST_2);
        let mut wide = super::WideGrid::from(&grid);
        assert_eq!(wide.simulate(&moves), 9021);
    }
}
