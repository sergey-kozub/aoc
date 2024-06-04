use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Grid {
    size: isize,
    data: Vec<Vec<u16>>,
    flashes: u32,
}
type Position = (isize, isize);

impl Grid {
    fn new(data: Vec<Vec<u16>>) -> Grid {
        Grid {
            size: data.len() as isize,
            data,
            flashes: 0,
        }
    }
    
    fn increment(&mut self, (i, j): Position) -> bool {
        let (y, x) = (i as usize, j as usize);
        self.data[y][x] += 1;
        self.data[y][x] > 9
    }
    
    fn adjacent(&self, (i, j): Position) -> Vec<Position> {
        let mut result = Vec::<Position>::new();
        for y in (i-1)..=(i+1) {
            if y < 0 || y >= self.size { continue };
            for x in (j-1)..=(j+1) {
                if x < 0 || x >= self.size { continue };
                if y != i || x != j { result.push((y, x)) };
            }
        }
        result
    }

    fn step(&mut self) {
        let mut flash = Vec::<Position>::new();
        let mut visited = HashSet::<Position>::new();
        for i in 0..self.size {
            for j in 0..self.size {
                if self.increment((i, j)) {
                    flash.push((i, j));
                    visited.insert((i, j));
                }
            }
        }
        while !flash.is_empty() {
            for pos in self.adjacent(flash.pop().unwrap()) {
                if self.increment(pos) && !visited.contains(&pos) {
                    flash.push(pos);
                    visited.insert(pos);
                }
            }
        }
        for (i, j) in visited {
            self.data[i as usize][j as usize] = 0;
            self.flashes += 1
        }
    }
    
    fn simulate(&mut self, steps: u32) -> u32 {
        for _ in 0..steps {
            self.step()
        }
        self.flashes
    }

    fn wait_all(&mut self) -> u32 {
        let mut steps: u32 = 0;
        loop {
            if self.data.iter().all(|x| x.iter().sum::<u16>() == 0) { break };
            self.step();
            steps += 1
        }
        steps
    }
}

fn main() {
    let input: Vec<Vec<u16>> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u16).collect()).collect();
    let mut grid = Grid::new(input);

    let n_flashes = grid.simulate(100);
    let n_steps = grid.wait_all() + 100;
    println!("{} {}", n_flashes, n_steps)
}
