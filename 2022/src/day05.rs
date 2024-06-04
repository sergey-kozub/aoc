use itertools::Itertools;
use std::fmt;

#[derive(Clone)]
struct Crane {
    state: Vec<Vec<u8>>,
    moves: Vec<(u8, u8, u8)>,
}

impl Crane {
    fn from(input: &str) -> Crane {
        let (s_state, s_moves) = input.split_once("\n\n").unwrap();
        let size = s_state.lines().last().unwrap().split_whitespace()
            .last().unwrap().parse::<usize>().unwrap();

        let mut state: Vec<Vec<u8>> = vec![vec![]; size];
        s_state.lines().rev().skip(1).for_each(|s| {
            s.bytes().skip(1).step_by(4).enumerate().for_each(|(k, v)| {
                if v != 32 { state[k].push(v) }
            });
        });

        let moves: Vec<(u8, u8, u8)> = s_moves.lines().map(|s| {
            s.split_whitespace().skip(1).step_by(2)
                .map(|s| s.parse::<u8>().unwrap()).collect_tuple().unwrap()
        }).collect();

        Crane { state, moves }
    }

    fn top(&self) -> String {
        self.state.iter().map(|a| {
            match a.len() {
                0 => ' ',
                n => a[n - 1] as char
            }
        }).join("")
    }

    fn execute(&mut self, rev: bool) -> String {
        for (count, from, to) in &self.moves {
            let src = &mut self.state[*from as usize - 1];
            let mut tail = src.split_off(src.len() - *count as usize);
            if rev { tail.reverse(); }
            self.state[*to as usize - 1].append(&mut tail);
        }
        self.top()
    }
}

impl fmt::Display for Crane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, line) in self.state.iter().enumerate() {
            write!(f, "{}:", index + 1)?;
            line.iter().for_each(|v| write!(f, " {}", *v as char).unwrap());
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn run(content: &str) {
    let mut crane = Crane::from(content);
    let mut copy = crane.clone();
    println!("{} {}", crane.execute(true), copy.execute(false));
}

#[cfg(test)]
mod tests {
    fn check(s: &str, t: &str, rev: bool) {
        let mut crane = super::Crane::from(s);
        assert_eq!(crane.execute(rev), t);
    }

    #[test]
    fn moves() {
        let example =
        "    [D]\n\
         [N] [C]\n\
         [Z] [M] [P]\n\
          1   2   3 \n\
         \n\
         move 1 from 2 to 1\n\
         move 3 from 1 to 3\n\
         move 2 from 2 to 1\n\
         move 1 from 1 to 2";

         check(example, "CMZ", true);
         check(example, "MCD", false);
    }
}
