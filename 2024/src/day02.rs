
fn parse(text: &str) -> Vec<Vec<i32>> {
    text.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }).collect()
}

fn is_safe(data: &[i32], dist: u32) -> bool {
    let (inc, dec) = data.windows(2).fold((0, 0), |a, x| {
        let ok = (x[0] - x[1]).abs() <= dist as i32;
        (a.0 + (ok && x[0] < x[1]) as u32, a.1 + (ok && x[0] > x[1]) as u32)
    });
    inc.max(dec) + 1 >= data.len() as u32
}

fn is_safe_rec(data: &[i32], dist: u32) -> bool {
    (0..data.len()).rev().any(|i| {
        let mut a = data.to_owned();
        a.remove(i);
        is_safe(&a, dist)
    })
}

pub fn run(content: &str) {
    let data = parse(content);
    let safe_1 = data.iter().filter(|x| is_safe(x, 3)).count();
    let safe_2 = data.iter().filter(|x| is_safe_rec(x, 3)).count();
    println!("{} {}", safe_1, safe_2);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9";

    #[test]
    fn small() {
        let safe = super::parse(TEST).into_iter()
            .filter(|x| super::is_safe(&x, 3));
        assert_eq!(safe.count(), 2);
    }
    
    #[test]
    fn large() {
        let safe = super::parse(TEST).into_iter()
            .filter(|x| super::is_safe_rec(&x, 3));
        assert_eq!(safe.count(), 4);
    }
}
