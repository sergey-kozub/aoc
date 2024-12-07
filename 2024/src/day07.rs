use std::collections::HashSet;

struct Equation {
    values: Vec<i64>,
    result: i64,
}

impl Equation {
    fn parse(text: &str) -> Self {
        let (l, r) = text.split_once(": ").unwrap();
        Self {
            values: r.split(' ').map(|x| x.parse::<i64>().unwrap()).collect(),
            result: l.parse::<i64>().unwrap(),
        }
    }

    fn solvable(&self, concat: bool) -> bool {
        let mut items = Vec::from_iter(self.values.iter().rev().copied());
        let mut options = HashSet::from([items.pop().unwrap()]);
        while let Some(v1) = items.pop() {
            let mut next = HashSet::new();
            for v2 in options {
                next.insert(v1 + v2);
                next.insert(v1 * v2);
                if concat {
                    let base = 10_i64.pow(if v1 > 0 {v1.ilog10() + 1} else {1});
                    next.insert(v1 + v2 * base);
                }
            }
            options = next;
        }
        options.contains(&self.result)
    }

    fn score(data: &[Self], concat: bool) -> i64 {
        data.iter().filter(|x| x.solvable(concat))
            .map(|x| x.result).sum::<i64>()
    }
}

pub fn run(content: &str) {
    let data = content.lines().map(Equation::parse).collect::<Vec<_>>();
    let v1 = Equation::score(&data, false);
    let v2 = Equation::score(&data, true);
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let solve = |s| super::Equation::parse(s).solvable(false);
        assert_eq!(solve("190: 10 19"), true);
        assert_eq!(solve("3267: 81 40 27"), true);
        assert_eq!(solve("83: 17 5"), false);
        assert_eq!(solve("156: 15 6"), false);
    }

    #[test]
    fn large() {
        let solve = |s| super::Equation::parse(s).solvable(true);
        assert_eq!(solve("156: 15 6"), true);
        assert_eq!(solve("7290: 6 8 6 15"), true);
        assert_eq!(solve("161011: 16 10 13"), false);
    }
}
