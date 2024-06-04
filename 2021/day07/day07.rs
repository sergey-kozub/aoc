use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt").expect("Error reading input")
        .trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    let cost_at = |pos: i32, calc: fn(i32) -> i32| -> i32 {
        input.iter().map(|x| calc((x - pos).abs())).sum()
    };
    let cost_min = |calc: fn(i32) -> i32| {
        (*input.iter().min().unwrap() ..= *input.iter().max().unwrap())
            .map(|x| cost_at(x, calc)).min().unwrap()
    };
    println!("{} {}", cost_min(|x| x), cost_min(|x| (x + 1) * x / 2))
}
