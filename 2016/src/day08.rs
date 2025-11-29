use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
enum Action {
    Rect(u32, u32),
    RotateColumn(u32, u32),
    RotateRow(u32, u32),
}

#[derive(Debug)]
struct Display {
  lit: HashSet<(u32, u32)>,
  size: (u32, u32),
}

impl Display {
    fn new(width: u32, height: u32) -> Self {
        Self { lit: HashSet::new(), size: (width, height) }
    }

    fn step(self, action: &Action) -> Self {
        let lit: HashSet<(u32, u32)> = match action {
            Action::Rect(w, h) => {
                self.lit.into_iter().chain(
                    (0..*w).flat_map(move |x|
                    (0..*h).map(move |y| (x, y)))
                ).collect()
            },
            Action::RotateColumn(col, n) => {
                self.lit.into_iter().map(|(x, y)| {
                    if x == *col {(x, (y + *n) % self.size.1)}
                    else {(x, y)}
                }).collect()
            },
            Action::RotateRow(row, n) => {
                self.lit.into_iter().map(|(x, y)| {
                    if y == *row {((x + *n) % self.size.0, y)}
                    else {(x, y)}
                }).collect()
            },
        };
        Self { lit, size: self.size }
    }

    fn run(self, actions: &[Action]) -> Self {
        actions.iter().fold(self, |a, b| a.step(b))
    }
}

fn parse(text: &str) -> Vec<Action> {
    let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_column = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    let re_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let parse = |x: &str| x.parse::<u32>().unwrap();
    text.lines().map(|line| {
        if let Some(c) = re_rect.captures(line) {
            return Action::Rect(parse(&c[1]), parse(&c[2]));
        }
        if let Some(c) = re_column.captures(line) {
            return Action::RotateColumn(parse(&c[1]), parse(&c[2]));
        }
        if let Some(c) = re_row.captures(line) {
            return Action::RotateRow(parse(&c[1]), parse(&c[2]));
        }
        panic!();
    }).collect()
}

pub fn run(content: &str) {
    let actions = parse(content);
    let res = Display::new(50, 6).run(&actions);
    println!("{}", res.lit.len());
    for y in 0..res.size.1 {
        let line = (0..res.size.0).map(|x|
            if res.lit.contains(&(x, y)) {'#'} else {'.'}
        ).collect::<String>();
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        const TEST: &str = "\
            rect 3x2\n\
            rotate column x=1 by 1\n\
            rotate row y=0 by 4\n\
            rotate column x=1 by 1";
        let result = super::Display::new(7, 3).run(&super::parse(TEST));
        assert_eq!(result.lit.len(), 6);
    }
}
