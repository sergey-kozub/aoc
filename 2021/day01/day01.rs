use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt").expect("Error reading input")
        .split("\n").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect();
    let reduce = |arr: &Vec<i32>| -> i32 { arr.windows(2).map(|x| (x[0] < x[1]) as i32).sum() };
    let windows: Vec<i32> = input.windows(3).map(|x| x.iter().sum()).collect();
    println!("{} {}", reduce(&input), reduce(&windows))
}
