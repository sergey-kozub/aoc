use std::fs;

fn most_common_bit(lines: &Vec<String>, index: usize) -> bool {
    let count: u32 = lines.iter().map(|s| (s.as_bytes()[index] == b'1') as u32).sum();
    return (count * 2 >= lines.len() as u32) as bool;
}

fn reduce_by_bits(lines: &Vec<String>, most_common: bool) -> u32 {
    let mut data = lines.clone();
    for idx in 0..lines[0].len() {
        let keep_ones = most_common_bit(&data, idx) == most_common;
        data.retain(|s| (s.as_bytes()[idx] == b'1') == keep_ones);
        if data.len() == 1 { break }
    }
    return u32::from_str_radix(&data.pop().unwrap(), 2).unwrap();
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(String::from).collect();
    let (_n, m) = (input.len(), input[0].len());

    let gamma: u32 = (0..m).map(|i| most_common_bit(&input, i))
        .rev().enumerate().map(|(i, x)| (x as u32) * (1 << i)).sum();
    let epsilon = gamma ^ ((1 << m) - 1);

    let generator = reduce_by_bits(&input, true);
    let scrubber = reduce_by_bits(&input, false);

    println!("{} {}", gamma * epsilon, generator * scrubber)
}
