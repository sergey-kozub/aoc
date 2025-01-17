
#[derive(Debug)]
struct Node {
    prev: usize,
    next: usize,
}

#[derive(Debug)]
struct Spinlock {
    nodes: Vec<Node>,
    position: usize,
}

impl Spinlock {
    fn new() -> Self {
        Self {
            nodes: vec![Node { prev: 0, next: 0 }],
            position: 0,
        }
    }

    fn advance(&mut self, steps: usize) {
        for _ in 0..steps {
            self.position = self.nodes[self.position].next;
        }
    }

    fn insert(&mut self) -> usize {
        let prev = self.position;
        let next = self.nodes[prev].next;
        let cur = self.nodes.len();
        self.nodes.push(Node { prev, next });
        self.nodes[prev].next = cur;
        self.nodes[next].prev = cur;
        cur
    }

    fn build(size: usize, steps: usize) -> Self {
        let mut inst = Self::new();
        for _ in 0..size {
            inst.advance(steps);
            inst.position = inst.insert();
        }
        inst
    }

    fn next(&self, pos: usize) -> usize {
        self.nodes[pos].next
    }
}

pub fn run(content: &str) {
    let input = content.parse::<usize>().unwrap();
    let spin_1 = Spinlock::build(2017, input);
    let spin_2 = Spinlock::build(50_000_000, input);
    println!("{} {}", spin_1.next(spin_1.position), spin_2.next(0));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let test = super::Spinlock::build(2017, 3);
        assert_eq!(test.next(test.position), 638);
    }
}
