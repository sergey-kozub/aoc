
#[derive(Debug)]
struct Disc {
    size: u64,
    start: u64,
}

impl Disc {
    fn parse(line: &str) -> Self {
        let a = line.split(' ').collect::<Vec<_>>();
        let size = a[3].parse::<u64>().unwrap();
        let start = a[11].trim_end_matches('.').parse::<u64>().unwrap();
        Self { size, start }
    }

    fn parse_all(text: &str) -> Vec<Self> {
        text.lines().map(Self::parse).collect()
    }

    fn position(&self, time: u64) -> u64 {
        let from = self.size - self.start;
        if time >= from {
            (time - from) % self.size
        } else {
            self.size + time - from
        }
    }
}

fn search(data: &[Disc], start: u64, step: u64) -> u64 {
    if data.is_empty() { return start - 1; }
    let disc = &data[0];
    for time in (start..).step_by(step as usize) {
        if disc.position(time) == 0 {
            return search(&data[1..], time + 1, step * disc.size) - 1;
        }
    }
    return u64::MAX;
}

pub fn run(content: &str) {
    let mut discs = Disc::parse_all(content);
    let res1 = search(&discs, 0, 1);
    discs.push(Disc { size: 11, start: 0 });
    let res2 = search(&discs, 0, 1);
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        Disc #1 has 5 positions; at time=0, it is at position 4.\n\
        Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn small() {
        let discs = super::Disc::parse_all(TEST);
        assert_eq!(super::search(&discs, 0, 1), 5);
    }
}
