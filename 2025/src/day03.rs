use std::cmp;

fn find_max(digits: &[u64], size: usize) -> u64 {
    let mut data = digits.to_vec();
    for i in 1..size {
        let max = data.iter().rev()
            .scan(0_u64, |m, n| { *m = cmp::max(*m, *n); Some(*m) })
            .collect::<Vec<u64>>();
        let base = 10_u64.pow(i as u32);
        data = max.iter().rev().skip(1).zip(digits.iter())
            .map(|(m, n)| n * base + m).collect();
    }
    *data.iter().max().unwrap()
}

fn sum_max(text: &str, size: usize) -> u64 {
    let parse = |c: char| c.to_digit(10).unwrap() as u64;
    text.lines()
        .map(|line| line.chars().map(parse).collect::<Vec<_>>())
        .map(|data| find_max(&data, size)).sum::<u64>()
}

pub fn run(content: &str) {
    println!("{} {}", sum_max(content, 2), sum_max(content, 12));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111";

    #[test]
    fn small() {
        assert_eq!(super::sum_max(TEST, 2), 357);
    }

    #[test]
    fn large() {
        assert_eq!(super::sum_max(TEST, 12), 3121910778619);
    }
}
