use std::fs;

fn score_1(ch: char) -> i32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_2(ch: char) -> i32 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn validate(chunk: &str) -> (bool, i64) {
    let mut stack = Vec::<char>::new();
    for ch in chunk.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => if stack.is_empty() || stack.pop().unwrap() != ch {
                return (false, score_1(ch) as i64)
            }
        }
    }
    return (true, stack.iter().map(|ch| score_2(*ch) as i64)
        .enumerate().map(|(i, v)| v * 5_i64.pow(i as u32)).sum())
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(String::from).collect();

    let mut part1: i64 = 0;
    let mut part2: Vec<i64> = Vec::new();
    for line in input {
        match validate(&line) {
            (false, s1) => part1 += s1,
            (true, s2) => part2.push(s2),
        }
    }
    part2.sort();
    println!("{} {}", part1, part2[part2.len() / 2])
}
