use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::ops::RangeInclusive;

type Coord = (i32, i32, i32);

#[derive(Debug)]
struct Droplet {
    scan: HashSet<Coord>,
}

impl Droplet {
    fn from(input: &str) -> Droplet {
        let scan: HashSet<Coord> = HashSet::from_iter(
            input.lines().map(|s| s.split(',').map(
                |x| x.parse::<i32>().unwrap()
            ).collect_tuple().unwrap()));
        Droplet { scan }
    }

    fn surface(&self) -> usize {
        let mut connected: usize = 0;
        for &(x, y, z) in &self.scan {
            if self.scan.contains(&(x + 1, y, z)) { connected += 1; }
            if self.scan.contains(&(x, y + 1, z)) { connected += 1; }
            if self.scan.contains(&(x, y, z + 1)) { connected += 1; }
        }
        self.scan.len() * 6 - connected * 2
    }

    fn outer_surface(&self) -> usize {
        let mut x_range: RangeInclusive<i32> = 0..=0;
        let mut y_range: RangeInclusive<i32> = 0..=0;
        let mut z_range: RangeInclusive<i32> = 0..=0;
        for &(x, y, z) in &self.scan {
            x_range = min(*x_range.start(), x - 1) ..= max(*x_range.end(), x + 1);
            y_range = min(*y_range.start(), y - 1) ..= max(*y_range.end(), y + 1);
            z_range = min(*z_range.start(), z - 1) ..= max(*z_range.end(), z + 1);
        }
        let is_valid = |(x, y, z): Coord| -> bool {
            x_range.contains(&x) && y_range.contains(&y) && z_range.contains(&z)
        };

        enum Type { Space(Coord), Lava, Outside }
        let visit = |p: Coord| -> Type {
            if !is_valid(p) {
                Type::Outside
            } else if self.scan.contains(&p) {
                Type::Lava
            } else {
                Type::Space(p)
            }
        };

        let mut queue: VecDeque<Coord> = VecDeque::from([
            (*x_range.start(), *y_range.start(), *z_range.start())
        ]);
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut result: usize = 0;

        while let Some((x, y, z)) = queue.pop_front() {
            if visited.contains(&(x, y, z)) { continue; }
            visited.insert((x, y, z));
            let adjacent = [visit((x + 1, y, z)), visit((x - 1, y, z)),
                            visit((x, y + 1, z)), visit((x, y - 1, z)),
                            visit((x, y, z + 1)), visit((x, y, z - 1))];
            for type_ in adjacent {
                match type_ {
                    Type::Space(x) => queue.push_back(x),
                    Type::Lava => result += 1,
                    _ => (),
                }
            }
        }
        result
    }
}

pub fn run(content: &str) {
    let inst = Droplet::from(content);
    println!("{} {}", inst.surface(), inst.outer_surface());
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { concat!(
        "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n",
        "2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n",
        "3,2,5\n2,1,5\n2,3,5")
    }

    #[test]
    pub fn lava() {
        let inst = super::Droplet::from(example());
        assert_eq!(inst.surface(), 64);
        assert_eq!(inst.outer_surface(), 58);
    }
}
