use std::collections::HashSet;

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<u8>>,
    pos: (usize, usize),
    dir: u8,
}

impl Grid {
    fn new(text: &str) -> Self {
        let mut pos = (0, 0);
        let data = text.lines().enumerate().map(|(i, line)| {
            line.chars().enumerate().map(|(j, ch)| match ch {
                '^' => { pos = (j, i); 2 },
                '#' => 1,
                '.' => 0,
                _ => panic!(),
            }).collect()
        }).collect();
        Self { data, pos, dir: 0 }
    }

    fn count_loop(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.data[y][x] != 0 { continue; }
                let mut test = self.clone();
                test.data[y][x] = 1;
                count += test.any(|t| t.1) as usize;
            }
        }
        count
    }

    fn width(&self) -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }
}

impl Iterator for Grid {
    type Item = ((usize, usize), bool);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.pos;
        let (x, y) = match self.dir {
            0 => if y > 0 {Some((x, y - 1))} else {None},
            1 => if x < self.width() - 1 {Some((x + 1, y))} else {None},
            2 => if y < self.height() - 1 {Some((x, y + 1))} else {None},
            3 => if x > 0 {Some((x - 1, y))} else {None},
            _ => panic!(),
        }?;
        match self.data[y][x] {
            1 => self.dir = (self.dir + 1) % 4,
            _ => self.pos = (x, y),
        };

        let val = &mut self.data[self.pos.1][self.pos.0];
        let mask = 1 << (self.dir + 1);
        let new = (*val & mask) == 0;
        *val |= mask;
        Some((self.pos, !new))
    }
}

pub fn run(content: &str) {
    let grid = Grid::new(content);
    let count = grid.count_loop();
    let trail = HashSet::<_>::from_iter(grid.map(|t| t.0));
    println!("{} {}", trail.len(), count);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...";

    #[test]
    fn small() {
        let grid = super::Grid::new(TEST);
        let trail = super::HashSet::<_>::from_iter(grid.map(|t| t.0));
        assert_eq!(trail.len(), 41);
    }

    #[test]
    fn large() {
        let grid = super::Grid::new(TEST);
        assert_eq!(grid.count_loop(), 6);
    }
}
