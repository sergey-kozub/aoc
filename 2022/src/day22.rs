use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Debug)]
enum Move {
    Forward(usize),
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Path {
    moves: Vec<Move>,
}

impl Path {
    fn from(input: &str) -> Path {
        let term = ['L', 'R'];
        let moves: Vec<Move> = input.split_inclusive(term).flat_map(|s| {
            let n = s.trim_end_matches(term).parse::<usize>().unwrap();
            let mut a = vec![Move::Forward(n)];
            if s.ends_with(term) {
                a.push(match s.chars().last().unwrap() {
                    'L' => Move::Left,
                    'R' => Move::Right,
                    _ => panic!()
                });
            }
            a.into_iter()
        }).collect();
        Path { moves }
    }
}

// ----- [2D] ------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Row {
    walls: Vec<u8>,
    start: usize,
}

impl Row {
    fn end(&self) -> usize {
        self.start + self.walls.len() - 1
    }

    fn is_valid(&self, x: usize) -> bool {
        x >= self.start && x <= self.end()
    }

    fn has_wall(&self, x: usize) -> bool {
        self.walls[x - self.start] != 0
    }
}

#[derive(Clone, Debug)]
struct Maze {
    rows: Vec<Row>,
}

impl Maze {
    fn from(input: &str) -> Maze {
        let rows: Vec<Row> = input.lines().map(|s| {
            let walls: Vec<u8> = s.trim().chars().map(|c| match c {
                '.' => 0, '#' => 1, _ => panic!()
            }).collect();
            let start = s.len() - s.trim_start().len() + 1;
            Row { walls, start }
        }).collect();
        Maze { rows }
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        y >= 1 && y <= self.rows.len() && self.rows[y - 1].is_valid(x)
    }

    fn iter(&self, path: &Path) -> MazeIter<'_> {
        MazeIter {
            maze: self,
            path: path.moves.iter().rev().cloned().collect(),
            pos: (self.rows[0].start, 1),
            dir: Direction::Right,
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            if row.start > 1 {
                write!(f, "{}", " ".repeat(row.start - 1))?;
            }
            writeln!(f, "{}", String::from_iter(row.walls.iter().map(|v|
                match v { 0 => '.', 1 => '#', _ => panic!() }
            )))?;
        }
        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
struct MazeIter<'a> {
    maze: &'a Maze,
    path: Vec<Move>,
    pos: (usize, usize),
    dir: Direction,
}

impl<'a> Iterator for MazeIter<'a> {
    type Item = (usize, usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop().map(|move_| {
            match move_ {
                Move::Forward(n) => {
                    let (x, y) = self.pos;
                    self.pos = match self.dir {
                        Direction::Up | Direction::Down => {
                            let mut ny = if matches!(self.dir, Direction::Up)
                                {y - 1} else {y + 1};
                            if !self.maze.is_valid(x, ny) {
                                let mut it = (1..=self.maze.rows.len()).filter_map(|i| {
                                    if self.maze.is_valid(x, i) {Some(i)} else {None}
                                });
                                ny = (if ny < y { it.last() } else { it.next() }).unwrap();
                            }
                            let row = &self.maze.rows[ny - 1];
                            (x, if !row.has_wall(x) {ny} else {y})
                        },
                        Direction::Left | Direction::Right => {
                            let row = &self.maze.rows[y - 1];
                            let nx = if matches!(self.dir, Direction::Left) {
                                if x > row.start {x - 1} else {row.end()}
                            } else {
                                if x < row.end() {x + 1} else {row.start}
                            };
                            (if !row.has_wall(nx) {nx} else {x}, y)
                        },
                    };
                    if n > 1 && (self.pos.0 != x || self.pos.1 != y) {
                        self.path.push(Move::Forward(n - 1));
                    }
                },
                Move::Left => self.dir = self.dir.rotate().rotate().rotate(),
                Move::Right => self.dir = self.dir.rotate(),
            };
            (self.pos.0, self.pos.1, self.dir)
        })
    }
}

type Position = (usize, usize);
struct Surface<'a> {
    maze: &'a Maze,
    visited: HashSet<Position>,
}

impl<'a> Surface<'a> {
    fn from(maze: &Maze) -> Surface<'_> {
        Surface { maze, visited: HashSet::new() }
    }

    fn search(&self, src: Position) -> Option<Vec<Position>> {
        let mut queue: Vec<Vec<Position>> = vec![vec![src]];
        let mut visited: HashSet<Position> = HashSet::from([src]);
        while let Some(path) = queue.pop() {
            let (x, y) = path[path.len() - 1];
            for (nx, ny) in [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)] {
                if self.maze.is_valid(nx, ny) && !visited.contains(&(nx, ny)) {
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    if !self.visited.contains(&(nx, ny)) {
                        return Some(new_path);
                    }
                    queue.push(new_path);
                    visited.insert((nx, ny));
                }
            }
        }
        None
    }

    fn explore(&mut self, mut pos: Position) -> Vec<(Direction, Position)> {
        let mut result: Vec<(Direction, Position)> = Vec::new();
        self.visited.insert(pos);
        while let Some(path) = self.search(pos) {
            for s in path.windows(2) {
                let dx = s[1].0 as isize - s[0].0 as isize;
                let dy = s[1].1 as isize - s[0].1 as isize;
                let dir = match (dx, dy) {
                    (0, -1) => Direction::Up,
                    (0, 1) => Direction::Down,
                    (-1, 0) => Direction::Left,
                    (1, 0) => Direction::Right,
                    _ => panic!()
                };
                pos = s[1];
                result.push((dir, pos));
                self.visited.insert(pos);
            }
        }
        result
    }

    fn to_index(&self, (x, y): Position) -> usize {
        (x - self.maze.rows[y - 1].start) +
            self.maze.rows[..y - 1].iter().map(|x| x.walls.len()).sum::<usize>()
    }

    fn to_position(&self, mut index: usize) -> Position {
        for (y, row) in self.maze.rows.iter().enumerate() {
            if index < row.walls.len() { return (row.start + index, y + 1); }
            index -= row.walls.len();
        }
        panic!()
    }
}

// ----- [3D] ------------------------------------------------------------------

// Sides: (0: front, 1: top, 2: right, 3: bottom, 4: left, 5: back)
const ROTATIONS: [([u8; 6], Direction); 4] = [
    ([3, 0, 2, 5, 4, 1], Direction::Up),
    ([1, 5, 2, 0, 4, 3], Direction::Down),
    ([2, 1, 5, 3, 0, 4], Direction::Left),
    ([4, 1, 0, 3, 5, 2], Direction::Right),
];

fn rotate<T: Copy + fmt::Debug>(cube: [T; 6], dir: Direction) -> [T; 6] {
    let map_ = ROTATIONS.iter().find(|x| x.1 == dir).unwrap().0;
    let res = Vec::from_iter(map_.iter().map(|&i| cube[i as usize]));
    res.try_into().unwrap()
}

fn reverse(d1: Direction, d2: Direction) -> bool {
    d1 == d2 ||
    (d1 == Direction::Right && d2 == Direction::Up) ||
    (d1 == Direction::Up && d2 == Direction::Right) ||
    (d1 == Direction::Left && d2 == Direction::Down) ||
    (d1 == Direction::Down && d2 == Direction::Left)
}

#[derive(Debug)]
struct Side {
    surface: Maze,
    connect: Vec<(Direction, usize)>,
    offset: Position,
}

#[derive(Debug)]
struct Cube {
    surface: Maze,
    sides: [Side; 6],
    size: usize,
}

impl Cube {
    fn from(input: &str, size: usize) -> Cube {
        let mut surface = Maze { rows: Vec::new() };
        let mut children = Vec::<Maze>::new();
        let maze = Maze::from(input);
        assert!(maze.rows.len() % size == 0);

        for (i, row) in maze.rows.into_iter().enumerate() {
            assert!(row.start % size == 1);
            assert!(row.end() % size == 0);
            let n = row.walls.len() / size;
            if i % size == 0 {
                surface.rows.push(Row {
                    walls: vec![1; n],
                    start: row.start / size + 1,
                });
                for _ in 0..n {
                    children.push(Maze { rows: Vec::new() });
                }
            }
            let p = children.len() - n;
            for j in 0..n {
                let m = &mut children[p + j];
                let a = &row.walls[j * size..(j + 1) * size];
                m.rows.push(Row {
                    walls: Vec::from_iter(a.iter().copied()),
                    start: 1,
                });
            }
        }
        assert_eq!(children.len(), 6);

        let mut cube = [Some(0), None, None, None, None, None];
        let mut inst = Surface::from(&surface);
        for (dir, pos) in inst.explore((surface.rows[0].start, 1)) {
            cube = rotate(cube, dir.rotate().rotate());
            let idx = inst.to_index(pos);
            if let Some(test) = cube[0] {
                assert_eq!(test, idx);
            } else {
                cube[0] = Some(idx);
            }
        }

        let mut last = inst.to_position(cube[0].unwrap());
        let sides: Vec<Side> = (0..6).map(|index| {
            let (x, y) = inst.to_position(index);
            inst.visited.clear();
            for (dir, pos) in inst.explore(last) {
                cube = rotate(cube, dir.rotate().rotate());
                last = pos;
                if pos == (x, y) { break; }
            }
            Side {
                surface: children[index].clone(),
                connect: vec![
                    (Direction::Up, cube[1].unwrap()),
                    (Direction::Down, cube[3].unwrap()),
                    (Direction::Left, cube[4].unwrap()),
                    (Direction::Right, cube[2].unwrap()),
                ],
                offset: ((x - 1) * size, (y - 1) * size),
            }
        }).collect();

        Cube { surface, sides: sides.try_into().unwrap(), size }
    }

    fn iter(&self, path: &Path) -> CubeIter<'_> {
        CubeIter {
            cube: self,
            path: path.moves.iter().rev().cloned().collect(),
            pos: (1, 1, 0),
            dir: Direction::Right,
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.surface)
    }
}

#[derive(Debug)]
struct CubeIter<'a> {
    cube: &'a Cube,
    path: Vec<Move>,
    pos: (usize, usize, usize),
    dir: Direction,
}

impl<'a> Iterator for CubeIter<'a> {
    type Item = (usize, usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop().map(|move_| {
            match move_ {
                Move::Forward(steps) => {
                    let (mut x, mut y, mut z) = self.pos;
                    (x, y) = match self.dir {
                        Direction::Up => (x, y - 1),
                        Direction::Down => (x, y + 1),
                        Direction::Left => (x - 1, y),
                        Direction::Right => (x + 1, y),
                    };
                    let (mut d, n) = (self.dir, self.cube.size);
                    if x == 0 || x > n || y == 0 || y > n {
                        let mut v = if x % (n + 1) != 0 {x} else {y};
                        let to = self.cube.sides[z].connect.iter().find(
                            |&x| x.0 == self.dir).unwrap().1;
                        let back = self.cube.sides[to].connect.iter().find(
                            |&x| x.1 == z).unwrap().0;
                        if reverse(d, back) { v = n - v + 1; }

                        (x, y) = match back {
                            Direction::Up => (v, 1),
                            Direction::Down => (v, n),
                            Direction::Left => (1, v),
                            Direction::Right => (n, v),
                        };
                        (z, d) = (to, back.rotate().rotate());
                    }
                    if !self.cube.sides[z].surface.rows[y - 1].has_wall(x) {
                        self.pos = (x, y, z);
                        self.dir = d;
                        if steps > 1 {
                            self.path.push(Move::Forward(steps - 1));
                        }
                    }
                },
                Move::Left => self.dir = self.dir.rotate().rotate().rotate(),
                Move::Right => self.dir = self.dir.rotate(),
            };
            let offset = self.cube.sides[self.pos.2].offset;
            (self.pos.0 + offset.0, self.pos.1 + offset.1, self.dir)
        })
    }
}

// ----- main ------------------------------------------------------------------

fn score<T>(iter: T) -> u64
where T: Iterator<Item = (usize, usize, Direction)> {
    let (x, y, d) = iter.last().unwrap();
    y as u64 * 1000 + x as u64 * 4 + match d {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
    }
}

pub fn run(content: &str) {
    let parts: Vec<&str> = content.trim_end().split("\n\n").collect();
    let maze = Maze::from(parts[0]);
    let cube = Cube::from(parts[0], 50);
    let path = Path::from(parts[1]);
    println!("{} {}", score(maze.iter(&path)), score(cube.iter(&path)));
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { r#"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.
"#.trim_matches('\n')
    }

    #[test]
    pub fn maze() {
        let maze = super::Maze::from(example());
        let path = super::Path::from("10R5L5R10L4R5L5");
        assert_eq!(super::score(maze.iter(&path)), 6032);

        let cube = super::Cube::from(example(), 4);
        assert_eq!(super::score(cube.iter(&path)), 5031);
    }
}
