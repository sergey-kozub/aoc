use std::collections::{HashMap, HashSet};

struct Patterns(Vec<String>);

fn parse(text: &str) -> (Patterns, Patterns) {
    let (l, r) = text.split_once("\n\n").unwrap();
    let a = l.split(", ").map(|s| s.to_string()).collect();
    let b = r.lines().map(|s| s.to_string()).collect();
    (Patterns(a), Patterns(b))
}

impl Patterns {
    fn can_build(&self, item: &str) -> bool {
        let mut visited = HashSet::new();
        let mut queue = vec![0];
        while let Some(pos) = queue.pop() {
            if pos == item.len() { return true; }
            let tail = &item[pos..];
            for pat in &self.0 {
                if tail.starts_with(pat) && visited.insert(pos + pat.len()) {
                    queue.push(pos + pat.len());
                }
            }
        }
        false
    }

    fn count(&self, item: &str) -> usize {
        let mut count = HashMap::from([(0, 1)]);
        for pos in 0..item.len() {
            let cur = match count.get(&pos) {
                Some(n) => *n,
                None => continue,
            };
            let tail = &item[pos..];
            for pat in &self.0 {
                if tail.starts_with(pat) {
                    count.entry(pos + pat.len())
                        .and_modify(|c| *c += cur)
                        .or_insert(cur);
                }
            }
        }
        *count.get(&item.len()).unwrap_or(&0)
    }
}

pub fn run(content: &str) {
    let (p1, p2) = parse(content);
    let v1 = p2.0.iter().filter(|s| p1.can_build(s)).count();
    let v2 = p2.0.iter().map(|s| p1.count(s)).sum::<usize>();
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        r, wr, b, g, bwu, rb, gb, br\n\n\
        brwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";

    #[test]
    fn small() {
        let (p1, p2) = super::parse(TEST);
        let res = p2.0.iter().map(|s| p1.can_build(s)).collect::<Vec<_>>();
        assert_eq!(res, vec![true, true, true, true, false, true, true, false]);
    }

    #[test]
    fn large() {
        let (p1, p2) = super::parse(TEST);
        let res = p2.0.iter().map(|s| p1.count(s)).collect::<Vec<_>>();
        assert_eq!(res, vec![2, 1, 4, 6, 0, 1, 2, 0]);
    }
}
