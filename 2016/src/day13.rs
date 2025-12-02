use std::collections::{HashSet, VecDeque};

struct Maze {
    design: i64,
}

impl Maze {
    fn is_wall(&self, x: i64, y: i64) -> bool {
        let n = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + self.design;
        n.count_ones() % 2 == 1
    }

    fn move_to(&self, dest: (i64, i64), limit: usize) -> usize {
        let start = (1_i64, 1_i64);
        let mut queue = VecDeque::from([(0_usize, start)]);
        let mut visited = HashSet::from([start]);
        while let Some((step, (x, y))) = queue.pop_front() {
            if (x, y) == dest {
                return step;
            }
            for pos in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                if self.is_wall(pos.0, pos.1) || visited.contains(&pos)
                    || pos.0 < 0 || pos.1 < 0 || step == limit {
                    continue;
                }
                queue.push_back((step + 1, pos));
                visited.insert(pos);
            }
        }
        visited.len()
    }
}

pub fn run(content: &str) {
    let maze = Maze { design: content.parse::<i64>().unwrap() };
    let res1 = maze.move_to((31, 39), usize::MAX);
    let res2 = maze.move_to((i64::MAX, 0), 50);
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::Maze { design: 10 }.move_to((7, 4)), 11);
    }
}
