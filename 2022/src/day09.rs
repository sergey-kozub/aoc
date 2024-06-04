use std::collections::HashSet;

#[derive(Debug)]
enum Direction { UP, DOWN, LEFT, RIGHT }

#[derive(Debug)]
struct Move {
    dir: Direction,
    step: usize,
}

#[derive(Debug)]
struct Bridge {
    moves: Vec<Move>,
}

impl Bridge {
    fn from(input: &str) -> Bridge {
        let moves = input.lines().map(|s| {
            let step = s[2..].parse::<usize>().unwrap();
            let dir = match s.chars().next().unwrap() {
                'U' => Direction::UP,
                'D' => Direction::DOWN,
                'L' => Direction::LEFT,
                'R' => Direction::RIGHT,
                _ => panic!("Unexpected input")
            };
            Move { dir, step }
        }).collect();
        Bridge { moves }
    }

    fn calc_move(&self, dx: i32, dy: i32) -> (i32, i32) {
        if dx.abs() <= 1 && dy.abs() <= 1 {
            (0, 0)
        } else {
            (dx.signum(), dy.signum())
        }
    }

    fn count_moves(&self, n: usize) -> usize {
        let mut pos = HashSet::<(i32, i32)>::new();
        let mut knots = vec![(0_i32, 0_i32); n + 1];
        pos.insert(knots[n]);
        for m in &self.moves {
            let (dx, dy) = match m.dir {
                Direction::UP => (0, 1),
                Direction::DOWN => (0, -1),
                Direction::LEFT => (-1, 0),
                Direction::RIGHT => (1, 0),
            };
            for _ in 0..m.step {
                knots[0] = (knots[0].0 + dx, knots[0].1 + dy);
                for k in 1..=n {
                    let dx = knots[k - 1].0 - knots[k].0;
                    let dy = knots[k - 1].1 - knots[k].1;
                    let (mx, my) = self.calc_move(dx, dy);
                    knots[k] = (knots[k].0 + mx, knots[k].1 + my);
                }
                pos.insert(knots[n]);
            }
        }
        pos.len()
    }
}

pub fn run(content: &str) {
    let inst = Bridge::from(content);
    println!("{} {}", inst.count_moves(1), inst.count_moves(9));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn moves() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let inst = super::Bridge::from(input);
        assert_eq!(inst.count_moves(1), 13);
        assert_eq!(inst.count_moves(9), 1);
    }
}
