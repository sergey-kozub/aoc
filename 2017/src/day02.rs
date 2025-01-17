
fn checksum(data: &[u32]) -> u32 {
    data.iter().max().unwrap() - data.iter().min().unwrap()
}

fn division(data: &[u32]) -> u32 {
    for (k, &v) in data.iter().enumerate() {
        for i in 0..k {
            let (m, n) = (data[i].max(v), data[i].min(v));
            if i != k && n != 0 && m % n == 0 { return m / n; }
        }
    }
    0
}

pub fn run(content: &str) {
    let data = content.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let v1 = data.iter().map(|a| checksum(&a)).sum::<u32>();
    let v2 = data.iter().map(|a| division(&a)).sum::<u32>();
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::checksum(&[5, 1, 9, 5]), 8);
        assert_eq!(super::checksum(&[7, 5, 3]), 4);
        assert_eq!(super::checksum(&[2, 4, 6, 8]), 6);
    }

    #[test]
    fn large() {
        assert_eq!(super::division(&[5, 9, 2, 8]), 4);
        assert_eq!(super::division(&[9, 4, 7, 3]), 3);
        assert_eq!(super::division(&[3, 8, 6, 5]), 2);
    }
}
