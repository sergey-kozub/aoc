use std::collections::HashMap;

const MOD: u64 = 16_777_216;

struct Bananas(u64);

impl Iterator for Bananas {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut acc = self.0;
        acc = ((acc << 6) ^ acc) % MOD;
        acc = ((acc >> 5) ^ acc) % MOD;
        acc = ((acc << 11) ^ acc) % MOD;
        self.0 = acc;
        Some(acc)
    }
}

fn explore(start: u64) -> HashMap<[i8; 4], u32> {
    let data = [(start % 10) as i8].into_iter().chain(
        Bananas(start).take(2000).map(|n| (n % 10) as i8)
    ).collect::<Vec<_>>();
    let change = data.windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>();
    let mut result = HashMap::new();
    for (k, v) in change.windows(4).enumerate() {
        let a: [i8; 4] = v.try_into().unwrap();
        if !result.contains_key(&a) {
            result.insert(a, data[k + 4] as u32);
        }
    }
    result
}

fn find_best(input: &[u64]) -> u32 {
    let sum = input.iter().fold(HashMap::new(), |mut acc, &n| {
        for (k, v) in explore(n) {
            acc.entry(k).and_modify(|c| *c += v).or_insert(v);
        }
        acc
    });
    *sum.values().max().unwrap()
}

pub fn run(content: &str) {
    let input = content.lines().map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let v1 = input.iter().map(|&n| Bananas(n).take(2000).last().unwrap())
        .sum::<u64>();
    let v2 = find_best(&input);
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let score = |n| super::Bananas(n).take(2000).last().unwrap();
        assert_eq!(score(1), 8685429);
        assert_eq!(score(10), 4700978);
        assert_eq!(score(100), 15273692);
        assert_eq!(score(2024), 8667524);
    }

    #[test]
    fn large() {
        let score = |n| super::explore(n).remove(&[-2, 1, -1, 3]);
        assert_eq!(score(1), Some(7));
        assert_eq!(score(2), Some(7));
        assert_eq!(score(3), None);
        assert_eq!(score(2024), Some(9));
    }
}
