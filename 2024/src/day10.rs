use std::collections::HashSet;

#[derive(Debug)]
struct Cell {
    height: u8,
    reach: HashSet<(usize, usize)>,
    rating: usize,
}

struct TopoMap {
    area: Vec<Vec<Cell>>,
}

impl TopoMap {
    fn parse(text: &str) -> Self {
        let area = text.lines().map(|line| {
            line.chars().map(|ch| {
                let digit = ch.to_digit(10).unwrap() as u8;
                Cell { height: digit, reach: HashSet::new(), rating: 0 }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        Self { area }
    }

    fn width(&self) -> usize { self.area[0].len() }
    fn height(&self) -> usize { self.area.len() }

    fn scan(&mut self) -> (usize, usize) {
        for n in (0..=9).rev() {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    if self.area[y][x].height != n { continue; }
                    let (mut acc, mut adj) = (HashSet::new(), 0);
                    let mut add = |c: &Cell| {
                        if c.height == n + 1 {
                            for t in &c.reach { acc.insert(*t); }
                            adj += c.rating;
                        }
                    };
                    if y > 0 { add(&self.area[y - 1][x]); }
                    if y < self.height() - 1 { add(&self.area[y + 1][x]); }
                    if x > 0 { add(&self.area[y][x - 1]); }
                    if x < self.width() - 1 { add(&self.area[y][x + 1]); }
                    if n == 9 { acc.insert((x, y)); adj = 1; }
                    self.area[y][x].reach = acc;
                    self.area[y][x].rating = adj;
                }
            }
        }
        self.area.iter().flat_map(|row| {
            row.iter().filter(|c| c.height == 0)
                .map(|c| (c.reach.len(), c.rating))
        }).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
    }
}

pub fn run(content: &str) {
    let res = TopoMap::parse(content).scan();
    println!("{} {}", res.0, res.1);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732";

    #[test]
    fn small() {
        let mut test = super::TopoMap::parse(TEST);
        assert_eq!(test.scan(), (36, 81));
    }
}
