use std::fs;

fn calculate(hist: &Vec<i64>, days: i32) -> i64 {
    let mut state = hist.clone();
    for _ in 0..days {
        let first = state[0];
        for i in 1..9 { state[i - 1] = state[i] };
        state[6] += first;
        state[8] = first;
    }
    state.iter().sum()
}

fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt").expect("Error reading input")
        .trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut hist: Vec<i64> = vec![0; 9];
    for day in input { hist[day as usize] += 1 };
    println!("{} {}", calculate(&hist, 80), calculate(&hist, 256))
}
