
#[derive(Clone, Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Clone, Debug)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum::<u64>(),
            Operation::Mul => self.numbers.iter().product::<u64>(),
        }
    }
}

fn parse(text: &str) -> Vec<Problem> {
    let lines = text.lines().map(|line| {
        line.split_whitespace().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (rows, columns) = (lines.len(), lines[0].len());
    (0..columns).map(|i| {
        let numbers = (0..rows - 1).map(|j| {
            lines[j][i].parse::<u64>().unwrap()
        }).collect::<Vec<_>>();
        let operation = match lines[rows - 1][i] {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => panic!(),
        };
        Problem { numbers, operation }
    }).collect()
}

fn parse_ceph(text: &str) -> Vec<Problem> {
    let lines = text.lines().collect::<Vec<_>>();
    let (rows, columns) = (lines.len(), lines[0].len());
    let mut last = lines[rows - 1].to_string();
    last += &" ".repeat(columns - last.len() + 1);
    let mut items = vec![];
    let mut slice = &last[..];
    while !slice.is_empty() {
        let op = match slice.chars().next().unwrap() {
            '+' => Operation::Add,
            '*' => Operation::Mul,
            _ => panic!(),
        };
        let p = slice[1..].find(|c| c != ' ').unwrap_or(slice.len() - 1);
        items.push((op, p));
        slice = &slice[p + 1..];
    }
    let mut pos = 0;
    (0..items.len()).map(|i| {
        let numbers = (0..items[i].1).map(|j| {
            (0..rows - 1).filter_map(|k| {
                let c = lines[k].as_bytes()[pos + j] as char;
                c.to_digit(10)
            }).fold(0, |acc, x| acc * 10 + x as u64)
        }).collect::<Vec<_>>();
        pos += items[i].1 + 1;
        Problem { numbers, operation: items[i].0.clone() }
    }).collect()
}

fn solve_all(data: &[Problem]) -> u64 {
    data.iter().map(|x| x.solve()).sum::<u64>()
}

pub fn run(content: &str) {
    let res1 = solve_all(&parse(content));
    let res2 = solve_all(&parse_ceph(content));
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

    #[test]
    fn small() {
        assert_eq!(super::solve_all(&super::parse(TEST)), 4277556);
    }

    #[test]
    fn large() {
        assert_eq!(super::solve_all(&super::parse_ceph(TEST)), 3263827);
    }
}
