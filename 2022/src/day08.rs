
#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<u8>>,
}

impl Forest {
    fn from(input: &str) -> Forest {
        let grid = input.lines().map(
            |s| s.as_bytes().iter().map(
            |c| c - 48).collect()).collect();
        Forest { grid }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let h = self.grid[y][x];
        (0..x).all(|i| self.grid[y][i] < h) ||
        (x+1..self.width()).all(|i| self.grid[y][i] < h) ||
        (0..y).all(|i| self.grid[i][x] < h) ||
        (y+1..self.height()).all(|i| self.grid[i][x] < h)
    }

    fn count_visible(&self) -> usize {
        (0..self.width()).flat_map(|x|
            (0..self.height()).map(move |y|
                self.is_visible(x, y) as usize
        )).sum()
    }

    fn score(&self, x: usize, y: usize) -> u32 {
        let h = self.grid[y][x];
        let scan = |dx: isize, dy: isize, n: usize| -> u32 {
            for i in 1..=(n as isize) {
                let px = (x as isize + dx * i) as usize;
                let py = (y as isize + dy * i) as usize;
                if self.grid[py][px] >= h { return i as u32; }
            }
            n as u32
        };
        scan(-1, 0, x) * scan(1, 0, self.width() - x - 1) *
        scan(0, -1, y) * scan(0, 1, self.height() - y - 1)
    }

    fn best_score(&self) -> u32 {
        (0..self.width()).flat_map(|x|
            (0..self.height()).map(move |y| self.score(x, y)
        )).max().unwrap()
    }
}

pub fn run(content: &str) {
    let inst = Forest::from(content);
    println!("{} {}", inst.count_visible(), inst.best_score());
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn trees() {
        let example = "\
            30373\n\
            25512\n\
            65332\n\
            33549\n\
            35390";
        let test = super::Forest::from(example);
        assert_eq!(test.count_visible(), 21);
        assert_eq!(test.best_score(), 8);
    }
}
