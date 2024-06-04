use crate::intcode::IntCode;
use libm::atan2;

type Position = (i32, i32);

fn between(a: Position, b: Position) -> Position {
    ((a.0 + b.0) / 2, (a.1 + b.1) / 2)
}

struct Scanner(String);

impl Scanner {
    fn get(&self, (x, y): Position) -> bool {
        let mut cpu = IntCode::from(&self.0);
        cpu.input.push_back(x as i64);
        cpu.input.push_back(y as i64);
        cpu.wait().unwrap() != 0
    }

    fn scan(&self, size: usize) -> usize {
        let mut count: usize = 0;
        for y in 0..size {
            for x in 0..size {
                let res = self.get((x as i32, y as i32));
                count += res as usize;
                print!("{}", if res {'#'} else {'.'});
            }
            println!();
        }
        count
    }

    fn search_range(&self, mut pos: [Position; 2]) -> Position {
        loop {
            let mid = between(pos[0], pos[1]);
            let idx = self.get(mid) as usize;
            if mid == pos[idx] { return pos[1]; }
            pos[idx] = mid;
        }
    }

    fn split_range(&self, pos: [Position; 2]) -> Option<[Position; 3]> {
        let mid = between(pos[0], pos[1]);
        if mid == pos[0] || mid == pos[1] {
            None
        } else if self.get(mid) {
            Some([pos[0], mid, pos[1]])
        } else {
            self.split_range([pos[0], mid]).or_else(
                || self.split_range([mid, pos[1]]))
        }
    }

    fn get_angles(&self) -> (f64, f64) {
        let big = 1_000;
        let [l, m, r] = self.split_range([(0, big), (big, 0)]).unwrap();
        let a = self.search_range([l, m]);
        let b = self.search_range([r, m]);
        (atan2(a.1 as f64, a.0 as f64), atan2(b.1 as f64, b.0 as f64))
    }

    fn search(&self, size: i32) -> i32 {
        let (a, _) = self.get_angles();
        let pos = |x: f64, d: f64| ((d * x.cos()).round() as i32,
                                    (d * x.sin()).round() as i32);
        let (mut l, mut r) = (size, 10 * size);
        while l < r - 1 {
            let m = (l + r) / 2;
            let p = pos(a, m as f64 / a.sin());
            assert!(self.get(p));
            if self.get((p.0 + size - 1, p.1 - size + 1)) {
                r = m;
            } else {
                l = m;
            }
        }
        let res = pos(a, r as f64 / a.sin());
        res.0 * 10_000 + res.1 - size + 1
    }
}

pub fn run(content: &str) {
    let scanner = Scanner(content.into());
    println!("{} {}", scanner.scan(50), scanner.search(100));
}
