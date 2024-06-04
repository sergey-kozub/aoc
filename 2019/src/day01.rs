
fn fuel(x: i32) -> i32 { x / 3 - 2 }
fn fuel_rec(x: i32) -> i32 {
    let r = fuel(x);
    if r > 0 {r + fuel_rec(r)} else {0}
}

pub fn run(content: &str) {
    let input: Vec<i32> = content.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let sum = |f: fn(i32) -> i32| input.iter().map(|&x| f(x)).sum::<i32>();
    println!("{} {}", sum(fuel), sum(fuel_rec))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::fuel(12), 2);
        assert_eq!(super::fuel(14), 2);
        assert_eq!(super::fuel(1969), 654);
        assert_eq!(super::fuel(100756), 33583);
    }

    #[test]
    fn part2() {
        assert_eq!(super::fuel_rec(14), 2);
        assert_eq!(super::fuel_rec(1969), 966);
        assert_eq!(super::fuel_rec(100756), 50346);
    }
}
