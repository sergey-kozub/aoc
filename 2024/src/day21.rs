use std::collections::HashMap;

#[derive(Debug)]
struct Keys(HashMap<(String, usize), usize>);

impl Keys {
    fn position(code: char) -> (i8, i8) {
        match code {
            'A' => (2, 1),
            '0' => (1, 1),
            '1'..='9' => {
                let n = code.to_digit(10).unwrap() as i8 - 1;
                (n % 3, n / 3 + 2)
            },
            '<' => (0, 0),
            'v' => (1, 0),
            '>' => (2, 0),
            '^' => (1, 1),
            _ => panic!(),
        }
    }

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn segment(dx: i8, dy: i8) -> String {
        assert!(dx == 0 || dy == 0);
        let ch = if dx < 0 {'<'} else if dx > 0 {'>'}
            else if dy < 0 {'v'} else {'^'};
        vec![ch; (dx + dy).abs() as usize].into_iter().collect()
    }

    fn measure(&mut self, path: String, depth: usize) -> usize {
        if depth == 0 { return path.len(); }
        let key = (path, depth);
        if let Some(n) = self.0.get(&key) { return *n; }
        let a = key.0.as_bytes();
        let size = a.iter().enumerate().map(|(k, v)| {
            let prev = if k == 0 {'A'} else {a[k - 1] as char};
            let (sx, sy) = Self::position(prev);
            let (ex, ey) = Self::position(*v as char);
            let (dx, dy) = (ex - sx, ey - sy);
            if dx != 0 && dy != 0 {
                let (s1, s2) = (Self::segment(dx, 0), Self::segment(0, dy));
                let v1 = if sy == 1 && ex == 0 {usize::MAX}
                    else { self.measure(s1.clone() + &s2 + "A", depth - 1) };
                let v2 = if sx == 0 && ey == 1 {usize::MAX}
                    else { self.measure(s2 + &s1 + "A", depth - 1) };
                v1.min(v2)
            } else {
                self.measure(Self::segment(dx, dy) + "A", depth - 1)
            }
        }).sum::<usize>();
        self.0.insert(key, size);
        size
    }

    fn score(&mut self, key: String, depth: usize) -> usize {
        let n = &key[..key.len() - 1].parse::<usize>().unwrap();
        let m = self.measure(key, depth);
        n * m
    }
}

pub fn run(content: &str) {
    let mut inst = Keys::new();
    let keys = content.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
    let mut total = |d| keys.iter().map(|s| inst.score(s.clone(), d))
        .sum::<usize>();
    println!("{} {}", total(3), total(26));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let score = |s: &str| super::Keys::new().measure(s.to_owned(), 3);
        assert_eq!(score("029A"), 68);
        assert_eq!(score("980A"), 60);
        assert_eq!(score("179A"), 68);
        assert_eq!(score("456A"), 64);
        assert_eq!(score("379A"), 64);
    }
}
