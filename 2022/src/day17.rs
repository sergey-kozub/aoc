use std::cmp;
use std::fmt;

const FIG_1: &str = "####";
const FIG_2: &str = ".#.\n###\n.#.";
const FIG_3: &str = "###\n..#\n..#";
const FIG_4: &str = "#\n#\n#\n#";
const FIG_5: &str = "##\n##";
const STEPS: usize = 1_000_000_000_000;

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn from(input: &str) -> Grid {
        let data = input.lines().map(|s| s.chars().map(|c| match c {
            '.' => 0, '#' => 1, _ => panic!()
        }).collect()).collect();
        Grid { data }
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.data.iter().rev() {
            writeln!(f, "{}", line.iter().map(|&x|
                if x != 0 {'#'} else {'.'}).collect::<String>())?;
        }
        fmt::Result::Ok(())
    }
}

#[derive(Clone)]
struct Tower {
    grid: Grid,
    width: usize,
    jets: String,
    index: usize,
}

impl Tower {
    fn create(jets: String, width: usize) -> Tower {
        Tower {
            grid: Grid { data: vec![] },
            width,
            jets,
            index: 0,
        }
    }

    fn get_shapes() -> Vec<Grid> {
        Vec::from_iter([FIG_1, FIG_2, FIG_3, FIG_4, FIG_5].map(Grid::from))
    }

    fn is_valid(&self, shape: &Grid, position: (i32, i32)) -> bool {
        let (x, y) = position;
        if y < 0 || x < 0 || x as usize > self.width - shape.width() {
            return false;
        }
        if y as usize >= self.grid.height() {
            return true;
        }
        (0..cmp::min(self.grid.height() - y as usize, shape.height())).all(|i| {
            self.grid.data[y as usize + i].iter().skip(x as usize)
                .zip(shape.data[i].iter()).all(|(a, b)| a + b <= 1)
        })
    }

    fn step(&self, shape: &Grid, position: (i32, i32), delta: i32) -> (i32, i32) {
        let (mut x, mut y) = position;
        if self.is_valid(shape, (x + delta, y)) { x += delta; }
        if self.is_valid(shape, (x, y - 1)) { y -= 1; }
        (x, y)
    }

    fn drop(&mut self, shape: &Grid) {
        let mut pos: (i32, i32) = (2, self.grid.height() as i32 + 3);
        loop {
            let prev = pos.1;
            let delta: i32 = match self.jets.as_bytes()[self.index] {
                60 => -1,  // '<'
                62 => 1,   // '>'
                _ => panic!()
            };
            pos = self.step(shape, pos, delta);
            self.index = (self.index + 1) % self.jets.len();
            if pos.1 == prev { break; }
        }
        shape.data.iter().enumerate().for_each(|(i, line)| {
            let y = pos.1 as usize + i;
            if y >= self.grid.height() {
                self.grid.data.push(vec![0; self.width]);
            }
            line.iter().enumerate().for_each(|(j, value)| {
                if *value != 0 {
                    let x = pos.0 as usize + j;
                    self.grid.data[y][x] = 1;
                }
            });
        });
    }

    fn simulate(&mut self, steps: usize) -> usize {
        let shapes = Tower::get_shapes();
        for i in 0..steps {
            let shape = &shapes[i % shapes.len()];
            self.drop(shape);
        }
        self.grid.height()
    }

    fn solve(&mut self, steps: usize) -> usize {
        let block = Tower::get_shapes().len() * self.jets.len();
        let mut deltas = Vec::<usize>::new();
        const N: usize = 3;
        loop {
            let prev = self.grid.height();
            let size = self.simulate(block);
            deltas.push(size - prev);
            for n in 1..=(deltas.len() / N) {
                let head = deltas.len() - n * N;
                let s = &deltas[head..];
                if (0..n).all(|i| (1..N).all(|j| s[n * j + i] == s[i])) {
                    let rsize: usize = deltas[head..head + n].iter().sum();
                    let m = (steps - head * block) / (n * block);
                    let x = steps - (head + m * n) * block;
                    return (m - N) * rsize + self.simulate(x);
                }
            }
        }
    }
}

pub fn run(content: &str) {
    let jets = String::from(content.trim_end());
    let tower = Tower::create(jets, 7);
    println!("{} {}", tower.clone().simulate(2022), tower.clone().solve(STEPS));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn tetris() {
        let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let tower = super::Tower::create(String::from(jets), 7);
        assert_eq!(tower.clone().simulate(2022), 3068);
        assert_eq!(tower.clone().solve(super::STEPS), 1_514_285_714_288);
    }
}
