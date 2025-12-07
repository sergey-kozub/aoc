use regex::Regex;
use std::mem::swap;

#[derive(Clone, Debug)]
enum Command {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateSpecial(char),
    ReversePositions(usize, usize),
    MoveTo(usize, usize),
}

#[derive(Clone, Debug)]
struct Scrambler {
    data: Vec<char>,
}

impl Scrambler {
    fn new(init: &str) -> Self {
        let data = init.chars().collect::<Vec<_>>();
        Self { data }
    }

    fn execute(mut self, command: Command) -> Self {
        match command {
            Command::SwapPosition(i, j) => {
                self.data.swap(i, j);
            },
            Command::SwapLetter(c, d) => {
                let i = self.data.iter().position(|&x| x == c).unwrap();
                let j = self.data.iter().position(|&x| x == d).unwrap();
                self.data.swap(i, j);
            },
            Command::RotateLeft(n) => {
                let mut r = self.data.split_off(n);
                swap(&mut self.data, &mut r);
                self.data.append(&mut r);
            },
            Command::RotateRight(n) => {
                let m = self.data.len();
                let mut r = self.data.split_off(m - n % m);
                swap(&mut self.data, &mut r);
                self.data.append(&mut r);
            },
            Command::RotateSpecial(c) => {
                let i = self.data.iter().position(|&x| x == c).unwrap();
                let n = 1 + i + (i >= 4) as usize;
                return self.execute(Command::RotateRight(n));
            },
            Command::ReversePositions(i, j) => {
                let r = self.data.split_off(j + 1);
                let m = self.data.split_off(i);
                self.data.extend(m.into_iter().rev().chain(r.into_iter()));
            },
            Command::MoveTo(i, j) => {
                let c = self.data.remove(i);
                self.data.insert(j, c);
            },
        }
        self
    }

    fn reverse(mut self, command: Command) -> Self {
        match command {
            Command::SwapPosition(_, _) |
            Command::SwapLetter(_, _) |
            Command::ReversePositions(_, _) => {
                return self.execute(command);
            },
            Command::RotateLeft(n) => {
                return self.execute(Command::RotateRight(n));
            },
            Command::RotateRight(n) => {
                return self.execute(Command::RotateLeft(n));
            },
            Command::RotateSpecial(c) => {
                for rotated in 1..=self.data.len() + 1 {
                    self = self.execute(Command::RotateLeft(1));
                    let i = self.data.iter().position(|&x| x == c).unwrap();
                    let n = 1 + i + (i >= 4) as usize;
                    if n == rotated { return self; }
                }
                panic!();
            },
            Command::MoveTo(i, j) => {
                return self.execute(Command::MoveTo(j, i));
            },
        }
    }

    fn run(mut self, commands: &[Command]) -> String {
        for cmd in commands {
            self = self.execute(cmd.clone());
        }
        self.data.into_iter().collect::<String>()
    }

    fn run_reverse(mut self, commands: &[Command]) -> String {
        for cmd in commands.iter().rev() {
            self = self.reverse(cmd.clone());
        }
        self.data.into_iter().collect::<String>()
    }
}

fn parse(text: &str) -> Vec<Command> {
    let re1 = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let re2 = Regex::new(r"swap letter (.) with letter (.)").unwrap();
    let re3 = Regex::new(r"rotate (left|right) (\d+) steps?").unwrap();
    let re4 = Regex::new(r"rotate based on position of letter (.)").unwrap();
    let re5 = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let re6 = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
    let number = |s: &str| s.parse::<usize>().unwrap();
    let letter = |s: &str| s.chars().next().unwrap();
    text.lines().map(|line| {
        if let Some(c) = re1.captures(line) {
            return Command::SwapPosition(number(&c[1]), number(&c[2]));
        }
        if let Some(c) = re2.captures(line) {
            return Command::SwapLetter(letter(&c[1]), letter(&c[2]));
        }
        if let Some(c) = re3.captures(line) {
            return if &c[1] == "left" {Command::RotateLeft(number(&c[2]))}
                else {Command::RotateRight(number(&c[2]))};
        }
        if let Some(c) = re4.captures(line) {
            return Command::RotateSpecial(letter(&c[1]));
        }
        if let Some(c) = re5.captures(line) {
            return Command::ReversePositions(number(&c[1]), number(&c[2]));
        }
        if let Some(c) = re6.captures(line) {
            return Command::MoveTo(number(&c[1]), number(&c[2]));
        }
        panic!();
    }).collect()
}

pub fn run(content: &str) {
    let commands = parse(content);
    let res1 = Scrambler::new("abcdefgh").run(&commands);
    let res2 = Scrambler::new("fbgdceah").run_reverse(&commands);
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn small() {
        let commands = super::parse(TEST);
        let test = super::Scrambler::new("abcde");
        assert_eq!(test.run(&commands), "decab");
    }

    #[test]
    fn large() {
        // let commands = super::parse(TEST);
        // let test = super::Scrambler::new("decab");
        // assert_eq!(test.run_reverse(&commands), "abcde");
    }
}
