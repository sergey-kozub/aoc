use std::fmt;
use std::fs;

#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid {
    fn from(text: &str) -> Grid {
        let lines: Vec<&str> = text.lines().collect();
        let width = lines[0].len();
        let height = lines.len();
        let data: Vec<u8> = lines.iter().flat_map(
            |s| s.chars().map(|ch| (ch == '#') as u8)).collect();
        Grid { width, height, data }
    }

    fn get(&self, i: isize, j: isize, outside: u8) -> u8 {
        if i >= 0 && i < self.height as isize && j >= 0 && j < self.width as isize {
            self.data[(i as usize) * self.width + j as usize]
        } else { outside }
    }

    fn fold(&self, i: isize, j: isize, v: u8) -> usize {
        let bits = vec![
            self.get(i - 1, j - 1, v), self.get(i - 1, j, v), self.get(i - 1, j + 1, v),
            self.get(i, j - 1, v), self.get(i, j, v), self.get(i, j + 1, v),
            self.get(i + 1, j - 1, v), self.get(i + 1, j, v), self.get(i + 1, j + 1, v)];
        bits.into_iter().fold(0_usize, |a, b| a * 2 + b as usize)
    }

    fn expand(&self, index: &[u8], rest: u8) -> Grid {
        let width = self.width + 2;
        let height = self.height + 2;
        let data = (0..height as isize).flat_map(|i| (0..width as isize).map(move |j| {
            index[self.fold(i - 1, j - 1, rest)]
        })).collect();
        Grid { width, height, data }
    }

    fn count(&self) -> usize {
        self.data.iter().filter(|&v| *v > 0).count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                result.push(if self.data[i * self.width + j] > 0 {'#'} else {'.'});
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

fn calc(grid: &Grid, index: &[u8], count: usize) -> usize {
    let mut result: Grid = (*grid).clone();
    let flip = index[0] == 1 && index[index.len() - 1] == 0;
    assert!(index[0] == 0 || flip, "infinite");
    for i in 0..count {
        result = result.expand(index, if flip { (i % 2) as u8 } else { 0 });
    }
    result.count()
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").expect("Error reading input")
        .split("\n\n").map(String::from).collect();
    let index: Vec<u8> = input[0].chars().map(|ch| (ch == '#') as u8).collect();
    let initial = Grid::from(&input[1]);
    let count = |n| calc(&initial, &index, n);
    println!("{} {}", count(2), count(50));
}
