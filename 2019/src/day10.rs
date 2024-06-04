use libm::atan2;
use std::collections::HashSet;

type Position = (i32, i32);

fn is_inside(a: Position, b: Position, p: Position) -> bool {
    let (x1, y1) = (a.0 - p.0, a.1 - p.1);
    let (x2, y2) = (p.0 - b.0, p.1 - b.1);
    (x1 * x2 > 0 && y1 * y2 > 0 && x1 * y2 == x2 * y1)
        || (x1 == 0 && x2 == 0 && y1 * y2 > 0)
        || (y1 == 0 && y2 == 0 && x1 * x2 > 0)
}

#[derive(Debug)]
struct AsteroidMap {
    positions: HashSet<Position>,
}

impl AsteroidMap {
    fn from(text: &str) -> AsteroidMap {
        let mut positions: HashSet<Position> = HashSet::new();
        text.lines().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, ch)| {
                if ch == '#' {
                    positions.insert((j as i32, i as i32));
                }
            });
        });
        AsteroidMap { positions }
    }

    fn is_visible(&self, a: Position, b: Position) -> bool {
        !self.positions.iter().any(|&p| is_inside(a, b, p))
    }

    fn count_visible(&self, a: Position) -> usize {
        self.positions.iter().filter(
            |&&p| p != a && self.is_visible(a, p)).count()
    }

    fn find_best(&self) -> (usize, Position) {
        self.positions.iter().map(
            |&p| (self.count_visible(p), p)).max().unwrap()
    }

    fn rank(&self, at: Position) -> Vec<Position> {
        let mut points: Vec<Position> = self.positions.iter().copied()
            .filter(|&p| p != at).collect();
        let angle = |p: Position| {
            let res = atan2((at.0 - p.0) as f64, (p.1 - at.1) as f64);
            if res == std::f64::consts::PI {-res} else {res}
        };
        let dist = |p: Position| (p.0 - at.0).abs() + (p.1 - at.1).abs();

        points.sort_by(|&a, &b| {
            let (k1, k2) = (angle(a), angle(b));
            if k1 != k2 {
                PartialOrd::partial_cmp(&k1, &k2).unwrap()
            } else {
                Ord::cmp(&dist(a), &dist(b))
            }
        });
        points
    }

    fn rotate(&self, at: Position) -> Vec<Position> {
        let mut points = self.rank(at);
        let mut result: Vec<Position> = Vec::new();
        while !points.is_empty() {
            let count = points.len();
            let mut remove: Vec<usize> = Vec::new();
            let mut index: usize = 0;

            while index < count {
                let pt = points[index];
                result.push(pt);
                remove.push(index);
                index += 1;
                while index < count && is_inside(at, points[index], pt) {
                    index += 1;
                }
            }
            for i in remove.into_iter().rev() {
                points.remove(i);
            }
        }
        result
    }
}

pub fn run(content: &str) {
    let amap = AsteroidMap::from(content);
    let result = amap.find_best();
    let n200 = amap.rotate(result.1)[199];
    println!("{} {}", result.0, n200.0 * 100 + n200.1)
}

#[cfg(test)]
mod tests {
    use super::AsteroidMap;

    #[test]
    fn part1() {
        assert_eq!((33, (5, 8)), AsteroidMap::from("\
            ......#.#.\n\
            #..#.#....\n\
            ..#######.\n\
            .#.#.###..\n\
            .#..#.....\n\
            ..#....#.#\n\
            #..#....#.\n\
            .##.#..###\n\
            ##...#..#.\n\
            .#....####").find_best());
        assert_eq!((35, (1, 2)), AsteroidMap::from("\
            #.#...#.#.\n\
            .###....#.\n\
            .#....#...\n\
            ##.#.#.#.#\n\
            ....#.#.#.\n\
            .##..###.#\n\
            ..#...##..\n\
            ..##....##\n\
            ......#...\n\
            .####.###.").find_best());
        assert_eq!((41, (6, 3)), AsteroidMap::from("\
            .#..#..###\n\
            ####.###.#\n\
            ....###.#.\n\
            ..###.##.#\n\
            ##.##.#.#.\n\
            ....###..#\n\
            ..#.#..#.#\n\
            #..#.#.###\n\
            .##...##.#\n\
            .....#.#..").find_best());
    }

    #[test]
    fn part2() {
        let res = AsteroidMap::from("\
            .#..##.###...#######\n\
            ##.############..##.\n\
            .#.######.########.#\n\
            .###.#######.####.#.\n\
            #####.##.#.##.###.##\n\
            ..#####..#.#########\n\
            ####################\n\
            #.####....###.#.#.##\n\
            ##.#################\n\
            #####.##.###..####..\n\
            ..######..##.#######\n\
            ####.##.####...##..#\n\
            .#####..#.######.###\n\
            ##...#.##########...\n\
            #.##########.#######\n\
            .####.#.###.###.#.##\n\
            ....##.##.###..#####\n\
            .#.#.###########.###\n\
            #.#.#.#####.####.###\n\
            ###.##.####.##.#..##").rotate((11, 13));
        assert_eq!(res[0], (11, 12));
        assert_eq!(res[1], (12, 1));
        assert_eq!(res[9], (12, 8));
        assert_eq!(res[19], (16, 0));
        assert_eq!(res[99], (10, 16));
        assert_eq!(res[199], (8, 2));
        assert_eq!(res[298], (11, 1));
    }
}
