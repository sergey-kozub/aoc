use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Assignment {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}

impl Assignment {
    fn from(s: &str) -> Assignment {
        let it = s.split(',').map(|s| {
            let b: Vec<i32> = s.split('-').map(
                |x| x.parse::<i32>().unwrap()).collect();
            RangeInclusive::new(b[0], b[1])
        });
        if let Some((first, second)) = it.collect_tuple() {
            Assignment { first, second }
        } else {
            panic!("unexpected size")
        }
    }

    fn contains(&self, reverse: bool, start: bool) -> bool {
        let a = if reverse { &self.second } else { &self.first };
        let b = if reverse { &self.first } else { &self.second };
        a.contains(if start { b.start() } else { b.end() })
    }

    fn overlap(&self) -> bool {
        let check = |r| self.contains(r, false) || self.contains(r, true);
        check(false) || check(true)
    }

    fn full_overlap(&self) -> bool {
        let check = |r| self.contains(r, false) && self.contains(r, true);
        check(false) || check(true)
    }
}

pub fn run(content: &str) {
    let input: Vec<Assignment> = content.lines().map(
        |s| Assignment::from(s)).collect();
    let overlaps: i32 = input.iter().map(|x| x.overlap() as i32).sum();
    let full_overlaps: i32 = input.iter().map(|x| x.full_overlap() as i32).sum();
    println!("{} {}", full_overlaps, overlaps);
}

#[cfg(test)]
mod tests {
    fn check(s: &str, full: bool, part: bool) {
        let inst = super::Assignment::from(s);
        assert_eq!(inst.full_overlap(), full);
        assert_eq!(inst.overlap(), part);
    }

    #[test]
    fn overlap() {
        check("2-4,6-8", false, false);
        check("2-3,4-5", false, false);
        check("5-7,7-9", false, true);
        check("2-8,3-7", true, true);
        check("6-6,4-6", true, true);
        check("2-6,4-8", false, true);
    }
}
