
type Position = (u64, u64);

#[derive(Debug)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Machine {
    fn parse(text: &str) -> Self {
        let parse = |s: &str| -> u64
            { s.split_once(&['+', '=']).unwrap().1.parse::<u64>().unwrap() };
        let a = text.lines().map(|line| {
            let (l, r) = line.split_once(", ").unwrap();
            (parse(l), parse(r))
        }).collect::<Vec<_>>();
        Self { button_a: a[0], button_b: a[1], prize: a[2] }
    }

    fn simple(&self) -> Option<u64> {
        let [(ax, ay), (bx, by), (px, py)] =
            [self.button_a, self.button_b, self.prize];
        let limit = (px / ax).min(py / ay);
        for n in 0..=limit {
            let (rx, ry) = (px - n * ax, py - n * ay);
            let m = (rx / bx).min(ry / by);
            if (rx, ry) == (m * bx, m * by) {
                return Some(n * 3 + m);
            }
        }
        None
    }

    fn complex(&self, add: u64) -> Option<u64> {
        let [(ax, ay), (bx, by), (px, py)] =
            [self.button_a, self.button_b, self.prize];
        let s = ((px + add) * by) as i64 - ((py + add) * bx) as i64;
        let t = (ax * by) as i64 - (bx * ay) as i64;
        if s % t != 0 || s.signum() != t.signum() { return None; }
        let n = (s / t) as u64;
        let (rx, ry) = (px + add - n * ax, py + add - n * ay);
        let m = rx / bx;
        if (rx, ry) != (m * bx, m * by) { return None; }
        Some(n * 3 + m)
    }
}

pub fn run(content: &str) {
    const OFFSET: u64 = 10_000_000_000_000;
    let claw = content.split("\n\n").map(Machine::parse).collect::<Vec<_>>();
    let v1 = claw.iter().filter_map(|x| x.simple()).sum::<u64>();
    let v2 = claw.iter().filter_map(|x| x.complex(OFFSET)).sum::<u64>();
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    const TEST_1: &str = "\
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400";
    const TEST_2: &str = "\
        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176";
    const TEST_3: &str = "\
        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450";
    const TEST_4: &str = "\
        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279";

    #[test]
    fn small() {
        let score = |s| super::Machine::parse(s).simple();
        assert_eq!(score(TEST_1), Some(280));
        assert_eq!(score(TEST_2), None);
        assert_eq!(score(TEST_3), Some(200));
        assert_eq!(score(TEST_4), None);
    }

    #[test]
    fn large() {
        let score = |s| super::Machine::parse(s).complex(10_000_000_000_000);
        assert_eq!(score(TEST_1), None);
        assert_eq!(score(TEST_2), Some(459_236_326_669));
        assert_eq!(score(TEST_3), None);
        assert_eq!(score(TEST_4), Some(416_082_282_239));
    }
}
