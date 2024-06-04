use std::fs;

#[derive(Debug)]
enum Move {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn parse_move(line: &str) -> Result<Move, &str> {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let step = parts[1].parse::<u32>().unwrap();
    match parts[0] {
        "up" => Ok(Move::Up(step)),
        "down" => Ok(Move::Down(step)),
        "forward" => Ok(Move::Forward(step)),
        _ => Err("Incorrect move"),
    }
}

fn process_1(moves: &Vec<Move>) -> u32 {
    let mut depth = 0;
    let mut position = 0;
    for move_ in moves {
        match move_ {
            Move::Up(n) => { depth -= n },
            Move::Down(n) => { depth += n },
            Move::Forward(n) => { position += n },
        }
    }
    depth * position
}

fn process_2(moves: &Vec<Move>) -> u32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut position = 0;
    for move_ in moves {
        match move_ {
            Move::Up(n) => { aim -= n },
            Move::Down(n) => { aim += n },
            Move::Forward(n) => { position += n; depth += aim * n },
        }
    }
    depth * position
}

fn main() {
    let input: Vec<Move> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(|s| parse_move(s).unwrap()).collect();
    println!("{} {}", process_1(&input), process_2(&input))
}
