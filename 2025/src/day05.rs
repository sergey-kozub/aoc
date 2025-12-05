use std::ops::RangeInclusive;

#[derive(Debug)]
struct Inventory {
    fresh: Vec<RangeInclusive<usize>>,
    items: Vec<usize>,
}

impl Inventory {
    fn parse(text: &str) -> Self {
        let (s1, s2) = text.split_once("\n\n").unwrap();
        let fresh = s1.lines().map(|line| {
            let (l, r) = line.split_once("-").unwrap();
            let l = l.parse::<usize>().unwrap();
            let r = r.parse::<usize>().unwrap();
            l..=r
        }).collect::<Vec<_>>();
        let items = s2.lines().map(|line| {
            line.parse::<usize>().unwrap()
        }).collect::<Vec<_>>();
        Self { fresh, items }
    }

    fn count_fresh(&self) -> usize {
        self.items.iter().filter(|&id| {
            self.fresh.iter().any(|r| r.contains(id))
        }).count()
    }

    fn clean_ranges(&self) -> usize {
        let mut ranges = self.fresh.clone();
        loop {
            let mut next = vec![];
            for r1 in ranges.iter().cloned() {
                let idx = next.iter().enumerate().filter_map(|(k, v)| {
                    if merge(&r1, v).is_some() {Some(k)} else {None}
                }).next();
                if let Some(i) = idx {
                    let r2 = next.swap_remove(i);
                    next.push(merge(&r1, &r2).unwrap());
                } else {
                    next.push(r1);
                }
            }
            if next.len() == ranges.len() {
                break;
            }
            ranges = next;
        }
        ranges.into_iter().map(|r| r.end() - r.start() + 1).sum::<usize>()
    }
}

fn merge(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>)
-> Option<RangeInclusive<usize>> {
    let (s1, e1) = (r1.start(), r1.end());
    let (s2, e2) = (r2.start(), r2.end());
    if *s1.max(s2) <= *e1.min(e2) + 1 {
        Some(*s1.min(s2) ..= *e1.max(e2))
    } else {None}
}

pub fn run(content: &str) {
    let inv = Inventory::parse(content);
    println!("{} {}", inv.count_fresh(), inv.clean_ranges());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        3-5\n10-14\n16-20\n12-18\n\n\
        1\n5\n8\n11\n17\n32";

    #[test]
    fn small() {
        assert_eq!(super::Inventory::parse(TEST).count_fresh(), 3);
    }

    #[test]
    fn large() {
        assert_eq!(super::Inventory::parse(TEST).clean_ranges(), 14);
    }
}
