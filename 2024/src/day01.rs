use std::collections::HashMap;

fn distance(a1: &[i32], a2: &[i32]) -> i32 {
    let mut a1: Vec<_> = a1.iter().cloned().collect();
    let mut a2: Vec<_> = a2.iter().cloned().collect();
    a1.sort();
    a2.sort();
    a1.iter().zip(a2.iter())
        .map(|(&x, &y)| (x - y).abs()).sum()
}

fn score(a1: &[i32], a2: &[i32]) -> i32 {
    let mut count = HashMap::<i32, i32>::new();
    for n in a2 { count.entry(*n).and_modify(|c| *c += 1).or_insert(1); }
    a1.iter().map(|n| *n * count.get(n).unwrap_or(&0)).sum()
}

pub fn run(content: &str) {
    let (a1, a2): (Vec<_>, Vec<_>) = content.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let mut next = || parts.next().unwrap().parse::<i32>().unwrap();
        (next(), next())
    }).unzip();
    println!("{} {}", distance(&a1, &a2), score(&a1, &a2));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let a1 = vec![3, 4, 2, 1, 3, 3];
        let a2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(super::distance(&a1, &a2), 11);
        assert_eq!(super::score(&a1, &a2), 31);
    }
}
