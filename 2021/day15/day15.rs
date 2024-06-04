use std::collections::BinaryHeap;
use std::fs;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid {
    fn new(lines: &Vec<String>) -> Grid {
        Grid {
            width: lines[0].len(),
            height: lines.len(),
            data: lines.iter().flat_map(|s| s.chars().map(
                |c| c.to_digit(10).unwrap() as u8)).collect(),
        }
    }

    fn lowest_risk(&self, span: usize) -> i32 {
        let (size, line) = (self.data.len() * span * span, self.width * span);
        let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut visited = vec![false; size];

        heap.push((0, 0));
        while let Some((val, idx)) = heap.pop() {
            if visited[idx] { continue; }
            if idx == size - 1 { return -val; }
            visited[idx] = true;

            let mut adjacent: Vec<usize> = Vec::new();
            if idx >= line { adjacent.push(idx - line); }
            if idx < size - line { adjacent.push(idx + line); }
            if idx % line != 0 { adjacent.push(idx - 1); }
            if idx % line != line - 1 { adjacent.push(idx + 1); }

            for next in adjacent {
                let (i, j) = (next / line, next % line);
                let (ty, py) = (i / self.height, i % self.height);
                let (tx, px) = (j / self.width, j % self.width);
                let raw = self.data[py * self.width + px] as i32;
                let risk = ((raw + tx as i32 + ty as i32) - 1) % 9 + 1;
                heap.push((val - risk, next));
            }
        }
        panic!("unexpected")
    }
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(String::from).collect();
    let grid = Grid::new(&input);
    println!("{} {}", grid.lowest_risk(1), grid.lowest_risk(5))
}
