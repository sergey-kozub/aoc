use std::collections::HashSet;

#[derive(Debug)]
struct Terrain {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Terrain {
    fn from(input: &str) -> Terrain {
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);
        let grid = input.lines().enumerate().map(|(y, s)| {
            s.chars().enumerate().map(|(x, c)| match c {
                'a'..='z' => (c as u8) - 97,
                'S' => { start = (x, y); 0 },
                'E' => { end = (x, y); 25 },
                _ => panic!("Incorrect input")
            }).collect()
        }).collect();
        Terrain { grid, start, end }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if x > 0 { result.push((x - 1, y)); }
        if x < self.width() - 1 { result.push((x + 1, y)); }
        if y > 0 { result.push((x, y - 1)); }
        if y < self.height() - 1 { result.push((x, y + 1)); }
        return result;
    }

    fn find_path(&self, rev: bool) -> usize {
        let mut visited = HashSet::<(usize, usize)>::new();
        let mut current = vec![if !rev {self.start} else {self.end}];
        let mut steps = 1_usize;
        'outer: while !current.is_empty() {
            let mut next = Vec::<(usize, usize)>::new();
            for (x, y) in current {
                let h = self.grid[y][x];
                for (nx, ny) in self.moves(x, y) {
                    let nh = self.grid[ny][nx];
                    let can_climb = if !rev { nh <= h + 1 } else { h <= nh + 1 };
                    if can_climb && !visited.contains(&(nx, ny)) {
                        next.push((nx, ny));
                        visited.insert((nx, ny));
                        let end = if !rev { (nx, ny) == self.end } else { nh == 0 };
                        if end { break 'outer; }
                    }
                }
            }
            current = next;
            steps += 1;
        }
        steps
    }
}

pub fn run(content: &str) {
    let inst = Terrain::from(content);
    println!("{} {}", inst.find_path(false), inst.find_path(true));
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { "\
        Sabqponm\n\
        abcryxxl\n\
        accszExk\n\
        acctuvwj\n\
        abdefghi"
    }

    #[test]
    pub fn climb() {
        let inst = super::Terrain::from(example());
        assert_eq!(inst.find_path(false), 31);
        assert_eq!(inst.find_path(true), 29);
    }
}
