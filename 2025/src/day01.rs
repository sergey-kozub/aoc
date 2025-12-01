
const DIAL_SIZE: u32 = 100;

#[derive(Clone, Debug)]
enum Dial {
    Left(u32),
    Right(u32),
}

struct DialIter<'a> {
    data: &'a [Dial],
    index: usize,
    position: u32,
}

impl<'a> Iterator for DialIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.data.len() {
            return None;
        }
        let skip = match self.data[self.index] {
            Dial::Left(n) => {
                let zero = self.position == 0;
                let skip = n / DIAL_SIZE + 1;
                self.position += skip * DIAL_SIZE - n;
                skip - (self.position > DIAL_SIZE) as u32 - zero as u32
            },
            Dial::Right(n) => {
                self.position += n;
                self.position / DIAL_SIZE
            },
        };
        self.index += 1;
        self.position %= DIAL_SIZE;
        Some((self.position, skip))
    }
}

fn count_zeros(data: &[Dial]) -> usize {
    DialIter { data, index: 0, position: 50 }
        .filter(|v| v.0 == 0).count()
}

fn count_skips(data: &[Dial]) -> usize {
    DialIter { data, index: 0, position: 50 }
        .map(|v| v.1).sum::<u32>() as usize
}

fn parse(text: &str) -> Vec<Dial> {
    text.lines().map(|line| match line.chars().nth(0) {
        Some('L') => Dial::Left(line[1..].parse::<u32>().unwrap()),
        Some('R') => Dial::Right(line[1..].parse::<u32>().unwrap()),
        _ => panic!(),
    }).collect()
}

pub fn run(content: &str) {
    let data = parse(content);
    println!("{} {}", count_zeros(&data), count_skips(&data));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn small() {
        let data = super::parse(TEST);
        assert_eq!(super::count_zeros(&data), 3);
    }

    #[test]
    fn large() {
        let data = super::parse(TEST);
        assert_eq!(super::count_skips(&data), 6);
    }
}
