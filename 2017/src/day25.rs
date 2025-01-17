use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
enum Move { Left, Right }

#[derive(Debug)]
struct Action {
    write: u8,
    direction: Move,
    state: usize,
}

#[derive(Debug)]
struct State(Action, Action);

#[derive(Debug)]
struct Turing {
    states: Vec<State>,
    current: usize,
    steps: usize,
    data: VecDeque<u8>,
    offset: usize,
    position: usize,
}

impl Action {
    fn parse(text: &str) -> Self {
        let re1 = Regex::new(r"Write the value (\d+)\.").unwrap();
        let re2 = Regex::new(r"Move one slot to the (\w+)\.").unwrap();
        let re3 = Regex::new(r"Continue with state ([A-Z])\.").unwrap();
        let state = |s: &str| (s.bytes().next().unwrap() - b'A') as usize;
        Self {
            write: re1.captures(text).unwrap()[1].parse::<u8>().unwrap(),
            direction: match &re2.captures(text).unwrap()[1] {
                "left" => Move::Left,
                "right" => Move::Right,
                _ => panic!(),
            },
            state: state(&re3.captures(text).unwrap()[1]),
        }
    }
}

impl State {
    fn parse(text: &str) -> Self {
        let re = Regex::new(
            r"If the current value is (\d+):\n(([^.]+\.){3})").unwrap();
        let mut matches = re.captures_iter(text).enumerate().map(|(i, c)| {
            let value = c[1].parse::<u8>().unwrap();
            assert_eq!(value, i as u8);
            Action::parse(&c[2])
        }).collect::<Vec<_>>();
        assert_eq!(matches.len(), 2);
        let (r, l) = (matches.pop().unwrap(), matches.pop().unwrap());
        Self(l, r)
    }
}

impl Turing {
    fn parse(text: &str) -> Self {
        let (head, body) = text.split_once("\n\n").unwrap();
        let re1 = Regex::new(r"Begin in state ([A-Z])\.").unwrap();
        let re2 = Regex::new(
            r"Perform a diagnostic checksum after (\d+) steps\.").unwrap();
        let state = |s: &str| (s.bytes().next().unwrap() - b'A') as usize;
        Self {
            states: body.split("\n\n").map(State::parse).collect(),
            current: state(&re1.captures(head).unwrap()[1]),
            steps: re2.captures(head).unwrap()[1].parse::<usize>().unwrap(),
            data: VecDeque::from([0]),
            offset: 0,
            position: 0,
        }
    }

    fn process(&mut self) -> usize {
        while self.next().is_some() {}
        self.data.iter().filter(|&&x| x != 0).count()
    }
}

impl Iterator for Turing {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        if self.steps == 0 { return None; }
        let state = &self.states[self.current];
        let act = if self.data[self.position] == 0 {&state.0} else {&state.1};
        self.data[self.position] = act.write;
        match act.direction {
            Move::Left => if self.position == 0 {
                self.data.push_front(0);
                self.offset += 1;
            } else { self.position -= 1; },
            Move::Right => if self.position == self.data.len() - 1 {
                self.data.push_back(0);
                self.position += 1;
            } else { self.position += 1; },
        }
        self.current = act.state;
        self.steps -= 1;
        Some(())
    }
}

pub fn run(content: &str) {
    let mut turing = Turing::parse(content);
    println!("{}", turing.process());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        Begin in state A.
        Perform a diagnostic checksum after 6 steps.

        In state A:
        If the current value is 0:
            - Write the value 1.
            - Move one slot to the right.
            - Continue with state B.
        If the current value is 1:
            - Write the value 0.
            - Move one slot to the left.
            - Continue with state B.

        In state B:
        If the current value is 0:
            - Write the value 1.
            - Move one slot to the left.
            - Continue with state A.
        If the current value is 1:
            - Write the value 1.
            - Move one slot to the right.
            - Continue with state A.";

    #[test]
    fn small() {
        let mut turing = super::Turing::parse(TEST);
        assert_eq!(turing.process(), 3);
    }
}
