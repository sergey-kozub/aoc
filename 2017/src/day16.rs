use std::collections::HashMap;

enum Move {
    Spin(u8),
    Exchange(u8, u8),
    Partner(u8, u8),
}

struct Dance(Vec<Move>);
struct Floor(Vec<u8>);

impl Dance {
    fn parse(text: &str) -> Self {
        let from_index = |s: &str| s.parse::<u8>().unwrap();
        let from_name = |s: &str| (s.as_bytes()[0] - b'a') as u8;

        let moves = text.split(",").map(|s| {
            match s.chars().next().unwrap() {
                's' => Move::Spin(from_index(&s[1..])),
                'x' => {
                    let (l, r) = &s[1..].split_once('/').unwrap();
                    Move::Exchange(from_index(l), from_index(r))
                },
                'p' => {
                    let (l, r) = &s[1..].split_once('/').unwrap();
                    Move::Partner(from_name(l), from_name(r))
                },
                _ => panic!(),
            }
        }).collect();
        Self(moves)
    }
}

impl Floor {
    fn new(n: u8) -> Self {
        Self((0..n).collect())
    }

    fn state(&self) -> String {
        self.0.iter().map(|x| (x + b'a') as char).collect()
    }

    fn update(&mut self, what: &Move) {
        match what {
            Move::Spin(n) => {
                let mut tail = self.0.split_off(self.0.len() - *n as usize);
                std::mem::swap(&mut self.0, &mut tail);
                self.0.append(&mut tail);
            },
            Move::Exchange(i, j) => {
                self.0.swap(*i as usize, *j as usize);
            },
            Move::Partner(a, b) => {
                let i = self.0.iter().position(|&x| x == *a).unwrap();
                let j = self.0.iter().position(|&x| x == *b).unwrap();
                self.0.swap(i, j);
            },
        }
    }

    fn run(mut self, dance: &Dance) -> Self {
        for item in &dance.0 { self.update(item); }
        self
    }

    fn run_many(mut self, dance: &Dance, count: usize) -> Self {
        let mut hash = HashMap::from([(self.state(), 0)]);
        for step in 1..=count {
            self = self.run(dance);
            let key = self.state();
            if let Some(idx) = hash.get(&key) {
                let rest = (count - step) % (step - *idx);
                for _ in 0..rest { self = self.run(dance); }
                break;
            }
            hash.insert(key, step);
        }
        self
    }
}

pub fn run(content: &str) {
    let dance = Dance::parse(content);
    let small = Floor::new(16).run(&dance);
    let large = Floor::new(16).run_many(&dance, 1_000_000_000);
    println!("{} {}", small.state(), large.state());
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let dance = super::Dance::parse("s1,x3/4,pe/b");
        assert_eq!(super::Floor::new(5).run(&dance).state(), "baedc");
    }

    #[test]
    fn large() {
        let dance = super::Dance::parse("s1,x3/4,pe/b");
        let v1 = super::Floor::new(5).run_many(&dance, 100);
        let mut v2 = super::Floor::new(5);
        for _ in 0..100 { v2 = v2.run(&dance); }
        assert_eq!(v1.state(), v2.state());
    }
}
