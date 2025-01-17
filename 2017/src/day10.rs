
struct KnotHash {
    data: Vec<u32>,
    skip: usize,
    position: usize,
}

impl KnotHash {
    fn new(n: u32) -> Self {
        Self { data: (0..n).collect(), skip: 0, position: 0 }
    }

    fn run(&mut self, steps: &[usize]) {
        let n = self.data.len();
        for &step in steps {
            if step > 0 { self.swap(self.position, step); }
            self.position = (self.position + step + self.skip) % n;
            self.skip += 1;
        }
    }

    fn swap(&mut self, pos: usize, len: usize) {
        let n = self.data.len();
        let (mut l, mut r) = (pos, (pos + len - 1) % n);
        while l != r {
            let (a, b) = (self.data[l], self.data[r]);
            self.data[l] = b;
            self.data[r] = a;
            l = (l + 1) % n;
            if l == r { break; }
            r = if r > 0 {r - 1} else {n - 1};
        }
    }

    fn dense(&self) -> String {
        assert_eq!(self.data.len() % 16, 0);
        let hex = |v: u8| if v < 10 {48 + v} else {87 + v};
        (&self.data).chunks(16).map(|a| {
            let b = a.iter().fold(0_u8, |x, y| x ^ (*y as u8));
            String::from_utf8(vec![hex(b / 16), hex(b % 16)]).unwrap()
        }).collect::<Vec<_>>().join("")
    }

    fn test(mut self, steps: &[usize]) -> String {
        for _ in 0..64 { self.run(steps); }
        self.dense()
    }
}

fn decode(text: &str) -> Vec<usize> {
    let mut res: Vec<_> = text.as_bytes().iter().map(|&x| x as usize).collect();
    res.append(&mut vec![17, 31, 73, 47, 23]);
    res
}

pub fn knot_hash(text: &str) -> String {
    KnotHash::new(256).test(&decode(text))
}

pub fn run(content: &str) {
    let steps = content.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut hasher = KnotHash::new(256);
    hasher.run(&steps);
    let score = hasher.data[0] * hasher.data[1];
    println!("{} {}", score, KnotHash::new(256).test(&decode(content)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let mut hasher = super::KnotHash::new(5);
        hasher.run(&[3, 4, 1, 5]);
        assert_eq!(hasher.data, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn large() {
        let test = |s| super::KnotHash::new(256).test(&super::decode(s));
        assert_eq!(test(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(test("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(test("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(test("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
