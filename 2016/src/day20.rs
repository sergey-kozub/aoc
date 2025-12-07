use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct Filter {
    ranges: Vec<RangeInclusive<u32>>,
}

impl Filter {
    fn parse(text: &str) -> Self {
        let ranges = text.lines().map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            let l = l.parse::<u32>().unwrap();
            let r = r.parse::<u32>().unwrap();
            l..=r
        }).collect::<Vec<_>>();
        Self { ranges }
    }

    fn lowest(&self) -> u32 {
        self.ranges.iter().filter_map(|r| {
            if *r.end() == u32::MAX { return None; }
            let v = *r.end() + 1;
            if self.ranges.iter().any(|t| t.contains(&v)) {None}
            else {Some(v)}
        }).min().unwrap()
    }

    fn allowed(&self) -> u32 {
        let mut valid = vec![0..=u32::MAX];
        for block in &self.ranges {
            let (s1, e1) = (*block.start(), *block.end());
            valid = valid.into_iter().flat_map(|range| {
                let (s2, e2) = (*range.start(), *range.end());
                let a = if e1.min(e2) >= s1.max(s2) {
                    let mut parts = vec![];
                    if s1 > s2 { parts.push(s2..=s1 - 1); }
                    if e1 < e2 { parts.push(e1 + 1..=e2); }
                    parts
                } else {vec![range]};
                a.into_iter()
            }).collect::<Vec<_>>();
        }
        valid.into_iter().map(|r| r.end() - r.start() + 1).sum()
    }
}

pub fn run(content: &str) {
    let filter = Filter::parse(content);
    println!("{} {}", filter.lowest(), filter.allowed());
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let test = super::Filter::parse("5-8\n0-2\n4-7");
        assert_eq!(test.lowest(), 3);
    }
}
