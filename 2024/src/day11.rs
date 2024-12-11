use std::collections::HashMap;

#[derive(Debug)]
struct Stones {
    count: HashMap<u64, usize>,
}

impl Stones {
    fn new(data: Vec<u64>) -> Self {
        let mut count = HashMap::new();
        for num in data {
            count.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }
        Self { count }
    }

    fn update(self) -> Self {
        let mut count = HashMap::new();
        for (key, val) in self.count {
            let mut add = |num|
                { count.entry(num).and_modify(|e| *e += val).or_insert(val); };
            let digits = if key != 0 {key.ilog10() + 1} else {1};
            if key == 0 {
                add(1);
            } else if digits % 2 == 0 {
                let div = 10_u64.pow(digits / 2);
                add(key / div);
                add(key % div);
            } else {
                add(key * 2024);
            }
        }
        Self { count }
    }

    fn repeat(mut self, count: usize) -> Self {
        for _ in 0..count { self = self.update(); }
        self
    }

    fn score(&self) -> usize {
        self.count.values().sum::<usize>()
    }
}

pub fn run(content: &str) {
    let data = content.split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let v1 = Stones::new(data.clone()).repeat(25).score();
    let v2 = Stones::new(data).repeat(75).score();
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let test = super::Stones::new(vec![125, 17]);
        assert_eq!(test.repeat(25).score(), 55312);
    }
}
