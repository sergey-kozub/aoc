
#[derive(Clone)]
struct Layer {
    index: usize,
    range: usize,
    current: usize,
    down: bool,
}

#[derive(Clone)]
struct Firewall(Vec<Layer>);

impl Firewall {
    fn parse(text: &str) -> Self {
        let layers = text.lines().map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            Layer {
                index: l.parse().unwrap(),
                range: r.parse().unwrap(),
                current: 0,
                down: true,
            }
        }).collect();
        Self(layers)
    }

    fn advance(&mut self) {
        for layer in &mut self.0 {
            if layer.down {
                layer.current += 1;
                if layer.current == layer.range - 1 { layer.down = false; }
            } else {
                layer.current -= 1;
                if layer.current == 0 { layer.down = true; }
            }
        }
    }

    fn score(&mut self, full: bool) -> Option<usize> {
        let mut time = 0;
        let mut severity = 0;
        for i in 0..self.0.len() {
            for _ in 0..self.0[i].index - time { self.advance(); }
            let layer = &self.0[i];
            time = layer.index;
            if layer.current == 0 {
                if !full { return None; }
                severity += layer.index * layer.range;
            }
        }
        Some(severity)
    }

    fn delay(&mut self) -> usize {
        let mut start = 0;
        loop {
            let mut copy = self.clone();
            if copy.score(false).is_some() { return start; }
            self.advance();
            start += 1;
        }
    }
}

pub fn run(content: &str) {
    let mut inst = Firewall::parse(content);
    let score = inst.clone().score(true).unwrap();
    println!("{} {}", score, inst.delay());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        0: 3\n\
        1: 2\n\
        4: 4\n\
        6: 4";

    #[test]
    fn small() {
        let mut inst = super::Firewall::parse(TEST);
        assert_eq!(inst.score(true), Some(24));
    }

    #[test]
    fn large() {
        let mut inst = super::Firewall::parse(TEST);
        assert_eq!(inst.delay(), 10);
    }
}
