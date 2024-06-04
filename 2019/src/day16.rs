use std::iter::repeat;

fn fft_pattern(size: usize) -> impl Iterator<Item = i8> {
    repeat(0).take(size)
        .chain(repeat(1).take(size))
        .chain(repeat(0).take(size))
        .chain(repeat(-1).take(size))
        .cycle()
        .skip(1)
}

fn as_vec(input: &str) -> Vec<u8> {
    input.trim_end().as_bytes().iter().map(|c| c - b'0').collect()
}

fn as_str(input: &[u8]) -> String {
    input.iter().map(|v| (v + b'0') as char).collect::<String>()
}

fn fft(input: &Vec<u8>) -> Vec<u8> {
    (1..=input.len()).map(|n| {
        (input.iter().zip(fft_pattern(n))
            .map(|(&a, b)| a as i32 * b as i32)
            .sum::<i32>().abs() % 10) as u8
    }).collect()
}

fn fft_n(mut input: Vec<u8>, steps: usize) -> Vec<u8> {
    for _ in 0..steps { input = fft(&input); }
    input
}

fn fft_str(input: &str, steps: usize) -> String {
    as_str(&fft_n(as_vec(input), steps))
}

fn search(input: &str, steps: usize) -> String {
    let index = input[..7].parse::<usize>().unwrap();
    let size = input.len() * 10_000;
    let mut result: Vec<u8> = (index..size).rev()
        .map(|i| input.as_bytes()[i % input.len()] - b'0')
        .collect();
    for _ in 0..steps {
        let mut sum = 0;
        result = result.iter().map(|v| {
            sum = (v + sum) % 10;
            sum
        }).collect()
    }
    result.iter().rev().take(8)
        .map(|v| (v + b'0') as char).collect()
}

pub fn run(content: &str) {
    println!("{} {}", &fft_str(content, 100)[..8], search(content.trim(), 100));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(&super::fft_str("80871224585914546619083218645595", 100)[..8], "24176176");
        assert_eq!(&super::fft_str("19617804207202209144916044189917", 100)[..8], "73745418");
        assert_eq!(&super::fft_str("69317163492948606335995924319873", 100)[..8], "52432133");
    }

    #[test]
    fn part2() {
        assert_eq!(&super::search("03036732577212944063491565474664", 100), "84462026");
        assert_eq!(&super::search("02935109699940807407585447034323", 100), "78725270");
        assert_eq!(&super::search("03081770884921959731165446850517", 100), "53553731");
    }
}
