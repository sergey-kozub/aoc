
fn check_1(value: i32) -> bool {
    let s: Vec<char> = value.to_string().chars().collect();
    s.windows(2).all(|x| x[0] <= x[1]) && s.windows(2).any(|x| x[0] == x[1])
}

fn check_2(mut value: i32) -> bool {
    if !check_1(value) { return false; }
    let mut hist = [0; 10];
    while value > 0 {
        hist[value as usize % 10] += 1;
        value /= 10;
    }
    hist.iter().any(|&x| x == 2)
}

pub fn run(content: &str) {
    let input: Vec<i32> = content.trim_end().split('-').map(
        |s| s.parse::<i32>().unwrap()).collect();
    let count = |f: fn(i32) -> bool|
        (input[0]..=input[1]).filter(|&x| f(x)).count();
    println!("{} {}", count(check_1), count(check_2))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::check_1(111111), true);
        assert_eq!(super::check_1(223450), false);
        assert_eq!(super::check_1(123789), false);
    }

    #[test]
    fn part2() {
        assert_eq!(super::check_2(112233), true);
        assert_eq!(super::check_2(123444), false);
        assert_eq!(super::check_2(111122), true);
    }
}
