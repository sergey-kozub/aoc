use itertools::Itertools;

#[derive(Clone, Debug)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    decision: (usize, usize),
    inspected: usize,
}

impl Monkey {
    fn from(input: &str) -> Monkey {
        let mut items = Vec::<u64>::new();
        let mut operation = Operation::Square;
        let mut divisor = 0_u64;
        let mut br_true = 0_usize;
        let mut br_false = 0_usize;
        for line in input.lines().skip(1) {
            let parts: (&str, &str) = line.split(": ").collect_tuple().unwrap();
            let last = parts.1.split_whitespace().last().unwrap();
            match parts.0.trim_start() {
                "Starting items" => {
                    items = parts.1.split(", ").map(|s|
                        s.parse::<u64>().unwrap()).collect();
                },
                "Operation" => {
                    let op: Vec<&str> = parts.1.split_whitespace().collect();
                    operation = match &op[3..] {
                        ["*", "old"] => Operation::Square,
                        ["*", n] => Operation::Mul(n.parse::<u32>().unwrap()),
                        ["+", n] => Operation::Add(n.parse::<u32>().unwrap()),
                        _ => panic!("Incorrect operation")
                    };
                },
                "Test" => divisor = last.parse::<u64>().unwrap(),
                "If true" => br_true = last.parse::<usize>().unwrap(),
                "If false" => br_false = last.parse::<usize>().unwrap(),
                _ => panic!("Incorrect token")
            }
        }
        Monkey {
            items, operation, divisor,
            decision: (br_true, br_false),
            inspected: 0,
        }
    }

    fn play(&mut self, level: u64, scale: u64) -> (u64, usize) {
        self.inspected += 1;
        let new = match self.operation {
            Operation::Add(n) => level + n as u64,
            Operation::Mul(n) => level * n as u64,
            Operation::Square => level * level,
        } / scale;
        let div = new % self.divisor == 0;
        (new, if div {self.decision.0} else {self.decision.1})
    }
}

#[derive(Clone, Debug)]
struct KeepAway {
    players: Vec<Monkey>,
    base: u64,
}

impl KeepAway {
    fn from(input: &str) -> KeepAway {
        let players: Vec<Monkey> = input.split("\n\n")
            .map(|s| Monkey::from(s)).collect();
        let base: u64 = players.iter().map(|p| p.divisor)
            .reduce(|a, b| a * b).unwrap();
        KeepAway { players, base }
    }

    fn play_round(&mut self, simple: bool) {
        let scale = if simple {3} else {1};
        for i in 0..self.players.len() {
            let items = self.players[i].items.split_off(0);
            for item in items {
                let (mut level, target) = self.players[i].play(item, scale);
                if !simple {
                    level %= self.base;
                }
                self.players[target].items.push(level);
            }
        }
    }

    fn monkey_business(&self) -> usize {
        let mut activity: Vec<usize> = self.players.iter()
            .map(|p| p.inspected).collect();
        activity.sort_by(|a, b| b.cmp(a));
        return activity[0] * activity[1];
    }

    fn play(&mut self, rounds: usize, simple: bool) -> usize {
        for _ in 0..rounds { self.play_round(simple); }
        self.monkey_business()
    }
}

pub fn run(content: &str) {
    let mut game = KeepAway::from(content);
    let mut game2 = game.clone();
    println!("{} {}", game.play(20, true), game2.play(10000, false));
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { "
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
        ".trim()
    }

    #[test]
    pub fn monkey() {
        let mut game = super::KeepAway::from(example());
        let mut game2 = game.clone();
        assert_eq!(game.play(20, true), 10_605);
        assert_eq!(game2.play(10000, false), 2_713_310_158);
    }
}
