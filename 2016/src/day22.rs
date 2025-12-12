use std::fmt;

#[derive(Clone, Debug)]
struct Node {
    coord: (i32, i32),
    used: i32,
    avail: i32,
}

#[derive(Clone, Debug)]
struct Grid {
    size: (i32, i32),
    data: Vec<Node>,
}

impl Node {
    fn parse(line: &str) -> Self {
        let as_int = |s: &str| s.parse::<i32>().unwrap();
        let a = line.split_whitespace().collect::<Vec<_>>();
        let b = a[0].split('-').collect::<Vec<_>>();
        let x = as_int(b[1].trim_start_matches('x'));
        let y = as_int(b[2].trim_start_matches('y'));
        let used = as_int(a[2].trim_end_matches('T'));
        let avail = as_int(a[3].trim_end_matches('T'));
        Self { coord: (x, y), used, avail }
    }
}

impl Grid {
    fn parse(text: &str) -> Self {
        let mut data = text.lines().skip(2).map(Node::parse)
            .collect::<Vec<_>>();
        let width = data.iter().map(|t| t.coord.0).max().unwrap() + 1;
        let height = data.iter().map(|t| t.coord.1).max().unwrap() + 1;
        data.sort_by_key(|t| (t.coord.1, t.coord.0));
        Self { size: (width, height), data }
    }

    fn crd2idx(&self, x: usize, y: usize) -> usize {
        y * self.size.0 as usize + x
    }

    fn idx2crd(&self, idx: usize) -> (usize, usize) {
        (idx % self.size.0 as usize, idx / self.size.0 as usize)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.chunks(self.size.0 as usize) {
            let line = row.iter().map(|t| {
                if t.used == 0 {'_'}
                else if t.used > 100 {'#'}
                else {'.'}
            }).collect::<String>();
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

fn count_pairs(grid: &Grid) -> usize {
    let mut count = 0;
    for (i, a) in grid.data.iter().enumerate() {
        for (j, b) in grid.data.iter().enumerate() {
            count += (i != j && a.used > 0 && a.used <= b.avail) as usize;
        }
    }
    count
}

fn count_reach(grid: &Grid) -> usize {
    let idx = grid.data.iter().position(|t| t.used == 0).unwrap();
    let (mut x, mut y) = grid.idx2crd(idx);
    let mut count = 0;
    while y > 0 {
        let above = &grid.data[grid.crd2idx(x, y - 1)];
        if above.used > 100 {
            x -= 1;
        } else {
            y -= 1;
        }
        count += 1;
    }
    let width = grid.size.0 as usize;
    let start = count + (width - x - 1);
    start + (width - 2) * 5
}

pub fn run(content: &str) {
    let grid = Grid::parse(content);
    println!("{}", grid);
    println!("{} {}", count_pairs(&grid), count_reach(&grid));
}
