
const K1: u64 = 16_807;
const K2: u64 = 48_271;
const MODULO: u64 = 2_147_483_647;
const MASK: u64 = 0xffff;

struct Generator {
    value: u64,
    factor: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.value = (self.value * self.factor) % MODULO;
        Some(self.value)
    }
}

fn count(v1: u64, v2: u64, d1: u64, d2: u64, count: usize) -> usize {
    let gen1 = (Generator { value: v1, factor: K1 }).filter(|&v| v % d1 == 0);
    let gen2 = (Generator { value: v2, factor: K2 }).filter(|&v| v % d2 == 0);
    gen1.zip(gen2).take(count)
        .filter(|(x, y)| (x & MASK) == (y & MASK))
        .count()
}

pub fn run(_: &str) {
    let (v1, v2) = (883, 879);
    println!("{} {}",
        count(v1, v2, 1, 1, 40_000_000),
        count(v1, v2, 4, 8, 5_000_000));
}
