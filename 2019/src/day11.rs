use crate::intcode::IntCode;
use std::collections::HashSet;
use std::iter::FromIterator;

type Position = (i32, i32);
enum Direction { Up, Right, Down, Left }

fn rotate(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn process(program: &str, initial: bool) -> (Vec<Position>, HashSet<Position>) {
    let mut pos: (i32, i32) = (0, 0);
    let mut dir = Direction::Up;
    let mut path: Vec<Position> = Vec::new();
    let mut paint: HashSet<Position> = HashSet::new();
    let mut inst = IntCode::from(program);
    if initial {
        paint.insert(pos);
    }

    loop {
        inst.input.push_back(paint.contains(&pos) as i64);
        match inst.wait() {
            Some(color) => {
                match color {
                    0 => paint.remove(&pos),
                    1 => paint.insert(pos),
                    _ => panic!("unknown color"),
                };
            },
            None => break,
        }
        match inst.wait() {
            Some(turn) => {
                match turn {
                    0 => dir = rotate(rotate(rotate(dir))),
                    1 => dir = rotate(dir),
                    _ => panic!("unknown turn"),
                }
            },
            None => panic!("missing turn"),
        }
        path.push(pos);
        pos = match dir {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
        };
    }
    (path, paint)
}

fn output(painted: &HashSet<Position>) -> String {
    let xmin = painted.iter().map(|p| p.0).min().unwrap();
    let xmax = painted.iter().map(|p| p.0).max().unwrap();
    let ymin = painted.iter().map(|p| p.1).min().unwrap();
    let ymax = painted.iter().map(|p| p.1).max().unwrap();

    (ymin..=ymax).map(|y| {
        (xmin..=xmax).map(|x| {
            if painted.contains(&(x, y)) {'x'} else {' '}
        }).collect::<String>()
    }).fold(String::new(), |a, b| a + &b + "\n")
}

pub fn run(content: &str) {
    let steps = HashSet::<Position>::from_iter(
        process(content, false).0.into_iter()).len();
    let image = output(&process(content, true).1);
    println!("{}\n{}", steps, image)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn simple() {
        let moves = vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0];
        let program = moves.iter().map(|&x| format!("104,{},", x)).join("") + "99";
        let result = super::process(&program, false);
        assert_eq!(result.0, vec![(0,0), (-1,0), (-1,1), (0,1), (0,0), (1,0), (1,-1)]);
        assert_eq!(result.1.len(), 4);
    }
}
