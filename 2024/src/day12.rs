use std::collections::HashSet;

struct Grid {
    data: Vec<Vec<u8>>,
}

struct Region {
    _label: char,
    points: Vec<(usize, usize)>,
}

impl Grid {
    fn parse(text: &str) -> Self {
        let data = text.lines().map(|line| {
            line.as_bytes().iter().map(|ch| ch - b'A').collect()
        }).collect();
        Self { data }
    }

    fn width(&self) -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }

    fn get_regions(&self) -> Vec<Region> {
        let mut regions = vec![];
        let mut visited = HashSet::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if visited.contains(&(x, y)) { continue; }
                let value = self.data[y][x];
                let mut queue = vec![(x, y)];
                let mut points = vec![];
                while let Some((x, y)) = queue.pop() {
                    if self.data[y][x] != value ||
                        !visited.insert((x, y)) { continue; }
                    if x > 0 { queue.push((x - 1, y)); }
                    if x < self.width() - 1 { queue.push((x + 1, y)); }
                    if y > 0 { queue.push((x, y - 1)); }
                    if y < self.height() - 1 { queue.push((x, y + 1)); }
                    points.push((x, y));
                }
                let _label = (value + b'A') as char;
                regions.push(Region { _label, points });
            }
        }
        regions
    }

    fn score(&self, sides: bool) -> u32 {
        self.get_regions().into_iter().map(|r| {
            r.area() * if sides {r.sides()} else {r.perimeter()}
        }).sum()
    }
}

impl Region {
    fn area(&self) -> u32 {
        self.points.len() as u32
    }

    fn perimeter(&self) -> u32 {
        let inner = HashSet::<(usize, usize)>::from_iter(
            self.points.iter().cloned());
        self.points.iter().map(|&(x, y)| {
            let l = x == 0 || !inner.contains(&(x - 1, y));
            let r = !inner.contains(&(x + 1, y));
            let u = y == 0 || !inner.contains(&(x, y - 1));
            let d = !inner.contains(&(x, y + 1));
            l as u32 + r as u32 + u as u32 + d as u32
        }).sum()
    }

    fn sides(&self) -> u32 {
        let inner = HashSet::<(usize, usize)>::from_iter(
            self.points.iter().cloned());
        let (mut h, mut v) = (vec![], vec![]);
        self.points.iter().for_each(|&(x, y)| {
            if x == 0 || !inner.contains(&(x - 1, y)) { v.push((x, y, true)); }
            if !inner.contains(&(x + 1, y)) { v.push((x + 1, y, false)); }
            if y == 0 || !inner.contains(&(x, y - 1)) { h.push((x, y, true)); }
            if !inner.contains(&(x, y + 1)) { h.push((x, y + 1, false)); }
        });
        h.sort_by_key(|(x, y, _)| (*y, *x));
        let next_x = |(x, y, z)| (x + 1, y, z);
        let dh = h.windows(2).filter(|a| next_x(a[0]) != a[1]).count();
        v.sort();
        let next_y = |(x, y, z)| (x, y + 1, z);
        let dv = v.windows(2).filter(|a| next_y(a[0]) != a[1]).count();
        dh as u32 + dv as u32 + 2
    }
}

pub fn run(content: &str) {
    let grid = Grid::parse(content);
    println!("{} {}", grid.score(false), grid.score(true));
}

#[cfg(test)]
mod tests {
    const TEST_1: &str = "\
        AAAA\n\
        BBCD\n\
        BBCC\n\
        EEEC";
    const TEST_2: &str = "\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
        OXOXO\n\
        OOOOO";
    const TEST_3: &str = "\
        RRRRIICCFF\n\
        RRRRIICCCF\n\
        VVRRRCCFFF\n\
        VVRCCCJFFF\n\
        VVVVCJJCFE\n\
        VVIVCCJJEE\n\
        VVIIICJJEE\n\
        MIIIIIJJEE\n\
        MIIISIJEEE\n\
        MMMISSJEEE";
    const TEST_4: &str = "\
        EEEEE\n\
        EXXXX\n\
        EEEEE\n\
        EXXXX\n\
        EEEEE";
    const TEST_5: &str = "\
        AAAAAA\n\
        AAABBA\n\
        AAABBA\n\
        ABBAAA\n\
        ABBAAA\n\
        AAAAAA";

    #[test]
    fn small() {
        let score = |s: &str| super::Grid::parse(s).score(false);
        assert_eq!(score(TEST_1), 140);
        assert_eq!(score(TEST_2), 772);
        assert_eq!(score(TEST_3), 1930);
    }

    #[test]
    fn large() {
        let score = |s: &str| super::Grid::parse(s).score(true);
        assert_eq!(score(TEST_1), 80);
        assert_eq!(score(TEST_2), 436);
        assert_eq!(score(TEST_3), 1206);
        assert_eq!(score(TEST_4), 236);
        assert_eq!(score(TEST_5), 368);
    }
}
