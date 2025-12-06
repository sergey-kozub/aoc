
#[derive(Clone)]
struct Ring {
    next: Vec<usize>,
}

impl Ring {
    fn new(count: usize) -> Self {
        let next = (0..count).map(|i| (i + 1) % count).collect::<Vec<_>>();
        Self { next }
    }

    fn run(mut self) -> usize {
        let mut cur = 0;
        while self.next[cur] != cur {
            let i = self.next[cur];
            let j = self.next[i];
            self.next[cur] = j;
            cur = j;
        }
        cur + 1
    }

    fn run_across(mut self) -> usize {
        let mut n = self.next.len();
        let (mut cur, mut opp) = (0, n / 2 - 1);
        while n > 1 {
            let i = self.next[opp];
            let j = self.next[i];
            self.next[opp] = j;
            cur = self.next[cur];
            if n % 2 != 0 { opp = j; }
            n -= 1;
        }
        cur + 1
    }
}

pub fn run(content: &str) {
    let n = content.parse::<usize>().unwrap();
    let res1 = Ring::new(n).run();
    let res2 = Ring::new(n).run_across();
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::Ring::new(5).run(), 3);
    }

    #[test]
    fn large() {
        assert_eq!(super::Ring::new(5).run_across(), 2);
    }
}
