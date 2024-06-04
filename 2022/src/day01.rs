use std::cmp;

fn top_n(input: &Vec<Vec<i32>>, n: usize) -> i32 {
    let mut sums: Vec<i32> = input.iter().map(|a| a.iter().sum()).collect();
    sums.sort_by_key(|v| cmp::Reverse(*v));
    sums[..n].iter().sum::<i32>()
}

pub fn run(content: &str) {
    let input: Vec<Vec<i32>> = content.trim_end().split("\n\n")
        .map(|a| a.lines().map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
    println!("{} {}", top_n(&input, 1), top_n(&input, 3))
}

#[cfg(test)]
mod tests {
    #[test]
    fn top_n() {
        let data = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000]];
        assert_eq!(super::top_n(&data, 1), 24000);
        assert_eq!(super::top_n(&data, 3), 45000);
    }
}
