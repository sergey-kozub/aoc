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
        let big = 100_000;
        let [l, m, r] = self.split_range([(0, big), (big, 0)]).unwrap();
        let a = self.search_range([l, m]);
        let b = self.search_range([r, m]);
        (atan2(a.1 as f64, a.0 as f64), atan2(b.1 as f64, b.0 as f64))
    }

    fn search(&self, size: i32) -> i32 {
        let (a, b) = self.get_angles();
        let c = (a + b) * 0.5;
        let pos = |x: f64, d: f64| ((d * x.cos()).round() as i32,
                                    (d * x.sin()).round() as i32);
        let (l, r) = (size, 1000 * size);
        let d = size - 1;
        for i in l..r {
            let p = pos(c, i as f64);
            assert!(self.get(p));
            let p2 = (-10..=0).flat_map(|e| [0, 1].iter().filter_map(move |f| {
                let p3 = (p.0 + e * f, p.1 + e * (1 - f));
                let left = self.get((p3.0 - d, p3.1));
                let top = self.get((p3.0, p3.1 - d));
                if left && top {Some(p3)} else {None}
            })).next();
            if let Some((x, y)) = p2 {
                return (x - d) * 10_000 + (y - d);
            }
        }
        0
    }
}

pub fn run(content: &str) {
    let scanner = Scanner(content.into());
    println!("{} {}", scanner.scan(50), scanner.search(100));
}
