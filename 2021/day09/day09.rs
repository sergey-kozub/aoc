use std::cmp::PartialOrd;
use std::collections::HashSet;
use std::fs;

struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}
type Position = (usize, usize);

impl<T: PartialOrd> Grid<T> {
    fn new(data: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            width: data[0].len(),
            height: data.len(),
            data,
        }
    }

    fn adjacent(&self, (i, j): Position) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();
        if i > 0 && i <= self.height { result.push((i - 1, j)) };
        if i < self.height - 1 { result.push((i + 1, j)) };
        if j > 0 && j <= self.width { result.push((i, j - 1)) };
        if j < self.width - 1 { result.push((i, j + 1)) };
        result
    }

    fn is_low(&self, pos: Position) -> bool {
        let value = &self.data[pos.0][pos.1];
        self.adjacent(pos).iter().all(|p| *value < self.data[p.0][p.1])
    }

    fn explore(&self, pos: Position, stop: &T, visited: &mut HashSet<Position>) {
        visited.insert(pos);
        for p in self.adjacent(pos) {
            if !visited.contains(&p) && self.data[p.0][p.1] != *stop {
                self.explore(p, stop, visited)
            }
        }
    }
}

fn main() {
    let input: Vec<Vec<u8>> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect();
    let grid = Grid::new(input);

    let mut low_points = Vec::<u8>::new();
    let mut basin_sizes = Vec::<usize>::new();
    let mut all_visited = HashSet::<Position>::new();
    for i in 0..grid.height {
        for j in 0..grid.width {
            if grid.is_low((i, j)) {
                low_points.push(grid.data[i][j])
            }
            if !all_visited.contains(&(i, j)) && grid.data[i][j] != 9 {
                let mut basin = HashSet::<Position>::new();
                grid.explore((i, j), &9, &mut basin);
                basin_sizes.push(basin.len());
                for p in basin { all_visited.insert(p); }
            }
        }
    }
    basin_sizes.sort();

    let score_1: u32 = low_points.iter().map(|x| 1 + *x as u32).sum();
    let score_2: usize = basin_sizes.iter().rev().take(3).product();
    println!("{} {}", score_1, score_2)
}
