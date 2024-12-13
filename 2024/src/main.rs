use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Task number is missing");

    let number: usize = args[1].parse().unwrap();
    let filename: String = if args.len() > 2 {
        args[2].clone()
    } else {
        format!("input/day{:02}.txt", number)
    };
    let content = fs::read_to_string(filename).expect("Error reading input");

    let days: Vec<fn(&str)> = vec![
        day01::run, day02::run, day03::run, day04::run, day05::run,
        day06::run, day07::run, day08::run, day09::run, day10::run,
        day11::run, day12::run, day13::run,
    ];
    days[number - 1](&content)
}
