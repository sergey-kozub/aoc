use regex::Regex;
use std::cmp;
use std::collections::HashMap;

type Bot = u32;
type Value = u32;

#[derive(Debug)]
enum BotOrOutput {
    Bot(u32),
    Output(u32),
}

#[derive(Debug)]
enum Rule {
    Value(Value, Bot),
    Gives(Bot, BotOrOutput, BotOrOutput),
}

#[derive(Debug)]
struct BotHolds(Vec<Value>);

#[derive(Debug)]
struct GameState {
    rules: Vec<Rule>,
    holds: HashMap<Bot, BotHolds>,
    outputs: HashMap<u32, Vec<Value>>,
    search: Option<(Value, Value)>,
}

impl GameState {
    fn new(rules: Vec<Rule>, search: Option<(Value, Value)>) -> Self {
        let mut holds = HashMap::<Bot, BotHolds>::new();
        let rules = rules.into_iter().filter(|x| match x {
            Rule::Value(v, i) => {
                holds.entry(*i).and_modify(|e| e.0.push(*v))
                    .or_insert(BotHolds(vec![*v]));
                false
            },
            _ => true,
        }).collect::<Vec<_>>();
        GameState { rules, holds, outputs: HashMap::new(), search }
    }

    fn step(&mut self) -> Option<Bot> {
        for rule in &self.rules {
            if let Rule::Gives(bot, low, high) = rule {
                if let Some(t) = self.holds.get(bot) {
                    if t.0.len() != 2 { continue; }
                    let h = cmp::max(t.0[0], t.0[1]);
                    let l = cmp::min(t.0[0], t.0[1]);
                    for (v, m) in [(l, low), (h, high)] {
                        match m {
                            BotOrOutput::Bot(i) => { self.holds.entry(*i)
                                .and_modify(|e| e.0.push(v))
                                .or_insert(BotHolds(vec![v])); ()},
                            BotOrOutput::Output(i) => { self.outputs.entry(*i)
                                .and_modify(|e| e.push(v))
                                .or_insert(vec![v]); ()},
                        }
                    }
                    if let Some((sl, sh)) = self.search {
                        if sl == l && sh == h {
                            return Some(*bot);
                        }
                    }
                    self.holds.remove(bot);
                    return None;
                }
            }
        }
        Some(u32::MAX)
    }

    fn run(&mut self) -> Bot {
        loop {
            if let Some(bot) = self.step() {
                return bot;
            }
        }
    }
}

fn parse(text: &str) -> Vec<Rule> {
    let re_value = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let re_gives = Regex::new(
        "bot (\\d+) gives low to (bot|output) (\\d+) \
         and high to (bot|output) (\\d+)").unwrap();
    let parse = |s: &str| s.parse::<u32>().unwrap();
    let parse_var = |s: &str, n: &str| {
        match s {
            "bot" => BotOrOutput::Bot(parse(n)),
            "output" => BotOrOutput::Output(parse(n)),
            _ => panic!(),
        }
    };

    text.lines().map(|line| {
        if let Some(c) = re_value.captures(line) {
            return Rule::Value(parse(&c[1]), parse(&c[2]))
        }
        if let Some(c) = re_gives.captures(line) {
            return Rule::Gives(
                parse(&c[1]), parse_var(&c[2], &c[3]), parse_var(&c[4], &c[5]))
        }
        panic!();
    }).collect()
}

pub fn run(content: &str) {
    let mut state_1 = GameState::new(parse(content), Some((17, 61)));
    let mut state_2 = GameState::new(parse(content), None);
    let res_1 = state_1.run();
    let _ = state_2.run();
    let res_2 = (0..3).map(|i| state_2.outputs.get(&i).unwrap()[0])
        .reduce(|a, b| a * b).unwrap();
    println!("{} {}", res_1, res_2);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn small() {
        let mut state = super::GameState::new(super::parse(TEST), Some((2, 5)));
        assert_eq!(state.run(), 2);
    }
}
