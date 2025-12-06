
#[derive(Clone)]
struct Maze {
    lines: Vec<Vec<u8>>,
}

impl Maze {
    fn new(line: &str) -> Self {
        let init = line.chars().map(|c| match c {
            '.' => 0_u8,
            '^' => 1_u8,
            _ => panic!(),
        }).collect::<Vec<_>>();
        Self { lines: vec![init] }
    }

    fn rows(&self) -> usize { self.lines.len() }
    fn _columns(&self) -> usize { self.lines[0].len() }

    fn next_line(&self) -> Vec<u8> {
        let last = &self.lines[self.rows() - 1];
        let mut result = vec![last[1]];
        result.extend(last.windows(3).map(|a| a[0] ^ a[2]));
        result.push(last[last.len() - 2]);
        result
    }

    fn grow(mut self, limit: usize) -> Self {
        while self.lines.len() < limit {
            self.lines.push(self.next_line());
        }
        self
    }

    fn count_safe(&self) -> usize {
        self.lines.iter()
            .flat_map(|line| line.iter().filter(|&&x| x == 0))
            .count()
    }
}

pub fn run(content: &str) {
    let maze = Maze::new(content.into());
    let res1 = maze.clone().grow(40).count_safe();
    let res2 = maze.grow(400_000).count_safe();
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let test = super::Maze::new(".^^.^.^^^^".into());
        assert_eq!(test.grow(10).count_safe(), 38);
    }
}
