use modinverse::modinverse;
use regex::Regex;

#[derive(Clone, Debug)]
enum Action {
    Cut(i64),
    DealIncrement(u64),
    DealStack,
}

impl Action {
    fn parse(line: &str) -> Option<Action> {
        let pattern = r"deal into new stack|deal with increment (\d+)|cut (-?\d+)";
        let re = Regex::new(pattern).unwrap();
        let m = re.captures(line)?;
        if let Some(cut) = m.get(2) {
            let value = cut.as_str().parse::<i64>().unwrap();
            return Some(Action::Cut(value));
        }
        if let Some(inc) = m.get(1) {
            let value = inc.as_str().parse::<u64>().unwrap();
            return Some(Action::DealIncrement(value));
        }
        return Some(Action::DealStack);
    }

    fn parse_all(text: &str) -> Vec<Action> {
        text.lines().map(|s| Action::parse(s).unwrap()).collect()
    }

    fn fold_pair(a1: Action, a2: Action, size: usize) -> Vec<Action> {
        match (&a1, &a2) {
            (Action::Cut(x), Action::Cut(y)) => {
                vec![Action::Cut((x + y) % size as i64)]
            },
            (Action::DealIncrement(x), Action::DealIncrement(y)) => {
                let n = ((*x as u128) * (*y as u128) % size as u128) as u64;
                vec![Action::DealIncrement(n)]
            },
            (Action::DealStack, Action::DealStack) => {
                vec![]
            },
            (Action::DealStack, Action::Cut(x)) => {
                vec![Action::Cut(-x), Action::DealStack]
            },
            (Action::Cut(x), Action::DealIncrement(y)) => {
                let n = ((*x as i128) * (*y as i128) % size as i128) as i64;
                vec![Action::DealIncrement(*y), Action::Cut(n)]
            },
            (Action::DealStack, Action::DealIncrement(x)) => {
                vec![Action::DealIncrement(*x), Action::Cut(1 - *x as i64), Action::DealStack]
            },
            _ => {
                vec![a1, a2]
            },
        }
    }

    fn fold_vec(mut actions: Vec<Action>, deck_size: usize) -> Vec<Action> {
        loop {
            let mut result: Vec<Action> = Vec::new();
            let initial_size = actions.len();
            for action in actions {
                if let Some(prev) = result.pop() {
                    result.append(&mut Action::fold_pair(prev, action, deck_size));
                } else {
                    result.push(action);
                }
            }
            if result.len() == initial_size {
                return result;
            }
            actions = result;
        }
    }

    fn fold_many(actions: &Vec<Action>, deck_size: usize, count: u64) -> Vec<Action> {
        let mut result: Option<Vec<Action>> = None;
        let mut acc = Action::fold_vec(actions.clone(), deck_size);
        for bit in 0..64 {
            if count & 1_u64 << bit != 0 {
                result = Some(match result {
                    Some(mut val) => {
                        val.append(&mut acc.clone());
                        Action::fold_vec(val, deck_size)
                    },
                    None => acc.clone(),
                })
            }
            acc.append(&mut acc.clone());
            acc = Action::fold_vec(acc, deck_size);
        }
        result.unwrap()
    }
}

#[derive(Clone, Debug)]
struct Deck(Vec<u32>);

impl Deck {
    fn new(size: usize) -> Deck {
        Deck((0..size as u32).collect())
    }

    fn cut(&self, n: i64) -> Deck {
        let m = if n > 0 {n} else {self.0.len() as i64 + n} as usize;
        let left = self.0.iter().copied().skip(m);
        let right = self.0.iter().copied().take(m);
        Deck(left.chain(right).collect())
    }

    fn deal_increment(&self, n: u64) -> Deck {
        let mut result = self.0.clone();
        let size = self.0.len();
        for i in 0..size {
            result[i * n as usize % size] = self.0[i];
        }
        Deck(result)
    }

    fn deal_stack(&self) -> Deck {
        Deck(self.0.iter().copied().rev().collect())
    }

    fn dispatch(&self, action: &Action) -> Deck {
        match action {
            Action::Cut(n) => self.cut(*n),
            Action::DealIncrement(n) => self.deal_increment(*n),
            Action::DealStack => self.deal_stack(),
        }
    }

    fn apply(&self, actions: &Vec<Action>) -> Deck {
        actions.iter().fold(self.clone(), |a, b| a.dispatch(b))
    }
}

#[derive(Debug)]
struct HugeDeck(usize);

impl HugeDeck {
    fn rev_cut(&self, pos: usize, n: i64) -> usize {
        (n + pos as i64 + self.0 as i64) as usize % self.0
    }

    fn rev_deal_increment(&self, pos: usize, n: u64) -> usize {
        let n = modinverse(n as i64, self.0 as i64).unwrap();
        (n as u128 * pos as u128 % self.0 as u128) as usize
    }

    fn rev_deal_stack(&self, pos: usize) -> usize {
        self.0 - pos - 1
    }

    fn rev_dispatch(&self, pos: usize, action: &Action) -> usize {
        match action {
            Action::Cut(n) => self.rev_cut(pos, *n),
            Action::DealIncrement(n) => self.rev_deal_increment(pos, *n),
            Action::DealStack => self.rev_deal_stack(pos),
        }
    }

    fn rev_apply(&self, pos: usize, actions: &Vec<Action>) -> usize {
        actions.iter().rev().fold(pos, |a, b| self.rev_dispatch(a, b))
    }
}

pub fn run(content: &str) {
    let actions = Action::parse_all(content);
    let deck = Deck::new(10007).apply(&actions);
    let huge_deck = HugeDeck(119_315_717_514_047);
    let actions_many = Action::fold_many(&actions, huge_deck.0, 101_741_582_076_661);

    let x_2019 = deck.0.iter().position(|&x| x == 2019).unwrap();
    let x_2020 = huge_deck.rev_apply(2020, &actions_many);
    println!("{} {}", x_2019, x_2020);
}

#[cfg(test)]
mod tests {
    fn example_1() -> (&'static str, Vec<u32>) { ("\
        deal with increment 7\n\
        deal into new stack\n\
        deal into new stack",
        vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7])
    }

    fn example_2() -> (&'static str, Vec<u32>) { ("\
        cut 6\n\
        deal with increment 7\n\
        deal into new stack",
        vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6])
    }

    fn example_3() -> (&'static str, Vec<u32>) { ("\
        deal with increment 7\n\
        deal with increment 9\n\
        cut -2",
        vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9])
    }

    fn example_4() -> (&'static str, Vec<u32>) { ("\
        deal into new stack\n\
        cut -2\n\
        deal with increment 7\n\
        cut 8\n\
        cut -4\n\
        deal with increment 7\n\
        cut 3\n\
        deal with increment 9\n\
        deal with increment 3\n\
        cut -1",
        vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6])
    }

    #[test]
    fn part1() {
        let test = |(text, expect): (&str, Vec<u32>)| {
            let actions = super::Action::parse_all(text);
            let deck = super::Deck::new(10);
            assert_eq!(expect, deck.apply(&actions).0);
        };
        test(example_1());
        test(example_2());
        test(example_3());
        test(example_4());
    }

    #[test]
    fn part2() {
        let test_fold = |text: &str, count: usize| {
            let actions = super::Action::parse_all(text);
            let deck_1 = (0..count).fold(
                super::Deck::new(10), |a, _| a.apply(&actions));
            let deck_2 = super::Deck::new(10).apply(
                &super::Action::fold_many(&actions, 10, count as u64));
            assert_eq!(deck_1.0, deck_2.0);
        };
        test_fold(example_1().0, 100);
        test_fold(example_2().0, 100);
        test_fold(example_3().0, 100);
        test_fold(example_4().0, 100);

        let test_rev = |(text, expect): (&str, Vec<u32>)| {
            let actions = super::Action::parse_all(text);
            let deck = super::HugeDeck(10);
            for i in 0..10 {
                let pos = deck.rev_apply(i, &actions);
                assert_eq!(pos as u32, expect[i]);
            }
        };
        test_rev(example_1());
        test_rev(example_2());
        test_rev(example_3());
        test_rev(example_4());
    }
}
