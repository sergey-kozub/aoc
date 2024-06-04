use crate::intcode::IntCode;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction { Up, Right, Down, Left }
type Position = (i32, i32);
type Robot = (Position, Direction);

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn right(&self) -> Direction {
        self.left().left().left()
    }

    fn step(&self, pos: &Position, dist: i32) -> Position {
        match self {
            Direction::Up => (pos.0, pos.1 - dist),
            Direction::Right => (pos.0 + dist, pos.1),
            Direction::Down => (pos.0, pos.1 + dist),
            Direction::Left => (pos.0 - dist, pos.1),
        }
    }

    fn relative(&self, other: Direction) -> Option<Step> {
        if *self == other {
            Some(Step::Move(0))
        } else if self.left() == other {
            Some(Step::Left)
        } else if self.right() == other {
            Some(Step::Right)
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq)]
enum Step {
    Move(usize),
    Left,
    Right,
}

impl Step {
    fn to_string(&self) -> String {
        match *self {
            Step::Move(n) => n.to_string(),
            Step::Left => "L".to_string(),
            Step::Right => "R".to_string(),
        }
    }
}

impl fmt::Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn normalize(steps: Vec<Step>) -> Vec<Step> {
    let mut result: Vec<Step> = Vec::new();
    let mut count: usize = 0;
    for step in steps {
        match step {
            Step::Left | Step::Right => {
                result.push(step);
                count = 0;
            },
            Step::Move(n) => {
                if count != 0 { result.pop(); }
                count += n;
                result.push(Step::Move(count));
            },
        }
    }
    result
}

struct Segment {
    path: Vec<Step>,
    visited: HashSet<Position>,
    state: Robot,
}

struct Grid {
    scaffold: HashSet<Position>,
    junction: HashSet<Position>,
    start: Robot,
}

impl Grid {
    fn from(text: &str) -> Grid {
        let mut scaffold: HashSet<Position> = HashSet::new();
        let mut robot: Option<Robot> = None;
        for (i, line) in text.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let pos = (j as i32, i as i32);
                let dir = match ch {
                    '^' => Some(Direction::Up),
                    '>' => Some(Direction::Right),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    _ => None,
                };
                if ch == '#' || dir.is_some() {
                    scaffold.insert(pos);
                }
                if dir.is_some() {
                    robot = Some((pos, dir.unwrap()));
                }
            }
        }
        let mut junction: HashSet<Position> = HashSet::new();
        for &(x, y) in &scaffold {
            if scaffold.contains(&(x + 1, y)) && scaffold.contains(&(x - 1, y)) &&
               scaffold.contains(&(x, y + 1)) && scaffold.contains(&(x, y - 1)) {
                junction.insert((x, y));
            }
        }
        Grid { scaffold, junction, start: robot.unwrap() }
    }

    fn from_intcode(program: &str) -> Grid {
        let mut cpu = IntCode::from(program);
        let mut text = String::new();
        while let Some(value) = cpu.wait() {
            text.push(value as u8 as char);
        }
        print!("{}", text);
        Grid::from(&text)
    }

    // Find direction to move into (except last position).
    fn next_cell(&self, current: &Robot, from: &Position) -> Option<Direction> {
        let mut dir = current.1;
        for _ in 0..4 {
            let pos = dir.step(&current.0, 1);
            if self.scaffold.contains(&pos) && pos != *from {
                return Some(dir);
            }
            dir = dir.left();
        }
        None
    }

    // Build path to the nearest junction (or final point).
    fn next_segment(&self, start: &Robot, initial: Direction, visited: &HashSet<Position>)
        -> Option<Segment> {
        let mut state = (initial.step(&start.0, 1), initial);
        if !self.scaffold.contains(&state.0) || visited.contains(&state.0) {
            return None;
        }

        let mut prev = start.0;
        let mut path: Vec<Step> = Vec::new();
        let mut visited: HashSet<Position> = HashSet::new();
        let mut steps: usize = 1;

        if start.1 != initial {
            path.push(start.1.relative(initial)?);
        }

        while let Some(dir) = self.next_cell(&state, &prev) {
            if dir != state.1 {
                path.push(Step::Move(steps));
                path.push(state.1.relative(dir).unwrap());
                steps = 0;
            }
            prev = state.0;
            visited.insert(prev);
            steps += 1;
            state = (dir.step(&prev, 1), dir);
            if self.junction.contains(&state.0) { break; }
        }

        if steps != 0 {
            path.push(Step::Move(steps));
        }
        Some(Segment { path, visited, state })
    }

    // Collect all paths that visit every scaffold.
    fn build_paths(&self) -> Vec<Vec<Step>> {
        let mut result: Vec<Vec<Step>> = Vec::new();
        let mut segments: Vec<Segment> = Vec::new();
        let mut checkpoints: Vec<(Robot, Direction)> = Vec::new();
        let mut visited: HashSet<Position> = HashSet::new();

        let init_dir = Direction::Up;
        let final_size = self.scaffold.len() - self.junction.len() - 2;
        checkpoints.push((self.start, init_dir));

        while let Some(&(robot, try_dir)) = checkpoints.last() {
            if let Some(mut segment) = self.next_segment(&robot, try_dir, &visited) {
                if visited.len() + segment.visited.len() == final_size {
                    let mut path: Vec<Step> = Vec::new();
                    for part in &segments {
                        path.append(&mut part.path.clone());
                    }
                    path.append(&mut segment.path);
                    result.push(normalize(path));
                } else {
                    for pos in &segment.visited {
                        visited.insert(*pos);
                    }
                    checkpoints.push((segment.state, init_dir));
                    segments.push(segment);
                    continue;
                }
            }
            while let Some((state, mut dir)) = checkpoints.pop() {
                dir = dir.left();
                if dir != init_dir {
                    checkpoints.push((state, dir));
                    break;
                }
                if let Some(segment) = segments.pop() {
                    for pos in segment.visited {
                        visited.remove(&pos);
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct Function<'a>(&'a [Step]);

impl<'a> Function<'a> {
    fn size(&self) -> usize {
        self.0.iter().map(|s| 1 + match s {
            Step::Left | Step::Right => 1,
            Step::Move(n) => if *n < 10 {1} else {2},
        }).sum::<usize>() - !self.0.is_empty() as usize
    }

    fn matches(&self, slice: &'a [Step]) -> bool {
        self.0.len() <= slice.len() &&
        self.0.iter().enumerate().all(|(k, v)| slice[k] == *v)
    }
}

#[derive(Debug)]
struct Solution {
    path: Vec<Step>,
    ranges: [Range<usize>; 3],
    routine: Vec<char>,
}

impl Solution {
    fn find_any(paths: Vec<Vec<Step>>, limit: usize) -> Solution {
        for path in paths {
            if let Some(solution) = Solution::find(path, limit) {
                return solution;
            }
        }
        panic!("unable to find a solution");
    }

    fn find(path: Vec<Step>, limit: usize) -> Option<Solution> {
        let mut ranges = [0..0, 0..0, 0..0];
        let function = |r: &Range<usize>| Function(&path[r.clone()]);
        let size = path.len() / 2;

        for p1 in 1..size {
            ranges[0] = 0 .. p1 * 2;
            let a = function(&ranges[0]);
            if a.size() > limit { break; }

            for p2 in (p1..size - 1).rev() {
                ranges[2] = p2 * 2 .. size * 2;
                let c = function(&ranges[2]);
                if c.size() > limit { break; }

                for p3 in p1..p2 {
                    for p4 in p3 + 1..p2 {
                        ranges[1] = p3 * 2 .. p4 * 2;
                        let b = function(&ranges[1]);
                        if b.size() > limit { break; }

                        if let Some(routine) = Solution::build(&path, [&a, &b, &c]) {
                            let rsize = routine.len() * 2 - 1;
                            if rsize <= limit {
                                return Some(Solution { path, ranges, routine });
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn build<'a>(input: &'a [Step], fns: [&Function<'a>; 3])
        -> Option<Vec<char>> {
        let mut res: Vec<char> = Vec::new();
        let mut idx: usize = 0;
        while idx < input.len() {
            let n = fns.iter().position(|f| f.matches(&input[idx..]))?;
            res.push((b'A' + n as u8) as char);
            idx += fns[n].0.len();
        }
        Some(res)
    }
}

fn run_solution(program: &str, solution: Solution) -> i64 {
    let mut input = solution.routine.iter().join(",") + "\n";
    for range in &solution.ranges {
        let mut it = solution.path[range.clone()].iter().map(|v| v.to_string());
        input.push_str(&(it.join(",") + "\n"));
    }
    input.push_str("n\n");

    let mut cpu = IntCode::from(program);
    cpu.set(0, 2);
    for ch in input.as_bytes() {
        cpu.input.push_back(*ch as i64);
    }
    cpu.run();
    *cpu.output.last().unwrap()
}

pub fn run(content: &str) {
    let grid = Grid::from_intcode(content);
    let score: i32 = grid.junction.iter().map(|(x, y)| x * y).sum();
    let solution = Solution::find_any(grid.build_paths(), 20);
    let secret = run_solution(content, solution);
    println!("{} {}", score, secret);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let grid = super::Grid::from("\
            ..#..........\n\
            ..#..........\n\
            #######...###\n\
            #.#...#...#.#\n\
            #############\n\
            ..#...#...#..\n\
            ..#####...^..");
        assert_eq!(grid.scaffold.len(), 39);
        assert_eq!(grid.junction.len(), 4);
        assert_eq!(grid.start, ((10, 6), super::Direction::Up));
    }

    #[test]
    fn part2() {
        let grid = super::Grid::from("\
            #######...#####\n\
            #.....#...#...#\n\
            #.....#...#...#\n\
            ......#...#...#\n\
            ......#...###.#\n\
            ......#.....#.#\n\
            ^########...#.#\n\
            ......#.#...#.#\n\
            ......#########\n\
            ........#...#..\n\
            ....#########..\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#####......");
        let solution = super::Solution::find_any(grid.build_paths(), 20);
        assert_eq!(solution.ranges, [0..8, 8..14, 32..36]);
        assert_eq!(solution.routine, ['A', 'B', 'C', 'B', 'A', 'C']);
    }
}
