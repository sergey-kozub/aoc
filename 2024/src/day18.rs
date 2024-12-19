use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
enum Cell { Empty, Wall }

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<Cell>>,
    drop: Vec<(usize, usize)>,
}

impl Grid {
    fn parse(size: usize, text: &str) -> Self {
        let data = vec![vec![Cell::Empty; size]; size];
        let drop = text.lines().map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        }).collect();
        Self { data, drop }
    }

    fn width(&self) -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }

    fn update(&mut self, count: usize) {
        for (x, y) in self.drop.iter().take(count) {
            self.data[*y][*x] = Cell::Wall;
        }
    }

    fn simple(&self) -> Option<usize> {
        let end = (self.width() - 1, self.height() - 1);
        let mut visited = HashSet::from([(0, 0)]);
        let mut queue = VecDeque::from([(0, 0, 0)]);
        let mut check = |x, y| {
            let row: &Vec<Cell> = &self.data[y];
            matches!(row[x], Cell::Empty) && visited.insert((x, y))
        };
        while let Some((x, y, d)) = queue.pop_front() {
            if (x, y) == end {
                return Some(d);
            }
            if x != 0 && check(x - 1, y) {
                queue.push_back((x - 1, y, d + 1));
            }
            if x != end.0 && check(x + 1, y) {
                queue.push_back((x + 1, y, d + 1));
            }
            if y != 0 && check(x, y - 1) {
                queue.push_back((x, y - 1, d + 1));
            }
            if y != end.1 && check(x, y + 1) {
                queue.push_back((x, y + 1, d + 1));
            }
        }
        None
    }

    fn search(&self) -> usize {
        let (mut l, mut r) = (0, self.drop.len());
        while l + 1 < r {
            let m = (l + r) / 2;
            let mut test = self.clone();
            test.update(m + 1);
            if test.simple().is_some() {l = m} else {r = m};
        }
        r
    }
}

pub fn run(content: &str) {
    let mut grid = Grid::parse(71, content);
    let (x, y) = grid.drop[grid.search()];
    grid.update(1024);
    println!("{} {x},{y}", grid.simple().unwrap());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n\
        2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n\
        6,1\n1,0\n0,5\n1,6\n2,0";

    #[test]
    fn small() {
        let mut inst = super::Grid::parse(7, TEST);
        inst.update(12);
        assert_eq!(inst.simple(), Some(22));
    }

    #[test]
    fn large() {
        let inst = super::Grid::parse(7, TEST);
        assert_eq!(inst.search(), 20);
    }
}
