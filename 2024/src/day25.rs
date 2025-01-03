
#[derive(Debug)]
enum Schematic {
    Lock([u8; 5]),
    Key([u8; 5]),
}

impl Schematic {
    fn parse(text: &str) -> Self {
        let data = text.lines().collect::<Vec<_>>();
        assert_eq!(data.len(), 7);
        let is_lock = data[0].chars().all(|c| c == '#');
        let is_key = data[6].chars().all(|c| c == '#');
        assert!(is_lock != is_key);
        let mut count = [0_u8; 5];
        for i in 0..5 {
            count[i] = (1..6).filter(|j| data[*j].as_bytes()[i] == b'#')
                .count() as u8;
        }
        if is_lock {Self::Lock(count)} else {Self::Key(count)}
    }

    fn parse_all(text: &str) -> Vec<Self> {
        text.split("\n\n").map(Self::parse).collect()
    }
}

fn count_fit(data: Vec<Schematic>) -> usize {
    let mut count = 0;
    let (locks, keys): (Vec<_>, Vec<_>) = data.into_iter()
        .partition(|x| matches!(x, Schematic::Lock(_)));
    for lock in &locks {
        let a = match lock { Schematic::Lock(x) => x, _ => panic!() };
        for key in &keys {
            let b = match key { Schematic::Key(x) => x, _ => panic!() };
            if (0..5).all(|i| a[i] + b[i] <= 5) {
                count += 1;
            }
        }
    }
    count
}

pub fn run(content: &str) {
    let data = Schematic::parse_all(content);
    println!("{}", count_fit(data));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        #####\n.####\n.####\n.####\n.#.#.\n.#...\n.....\n\n\
        #####\n##.##\n.#.##\n...##\n...#.\n...#.\n.....\n\n\
        .....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####\n\n\
        .....\n.....\n#.#..\n###..\n###.#\n###.#\n#####\n\n\
        .....\n.....\n.....\n#....\n#.#..\n#.#.#\n#####";

    #[test]
    fn small() {
        let data = super::Schematic::parse_all(TEST);
        assert_eq!(super::count_fit(data), 3);
    }
}
