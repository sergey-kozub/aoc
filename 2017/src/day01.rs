
fn sum_matching_next(text: &str) -> u32 {
    let conv = |x| (x - 48) as u32;
    let bytes = text.as_bytes();
    bytes.windows(2)
        .filter(|a| a[0] == a[1]).map(|a| conv(a[0])).sum::<u32>() +
    if bytes.first() == bytes.last() {conv(bytes[0])} else {0}
}

fn sum_matching_half(text: &str) -> u32 {
    let bytes = text.as_bytes();
    let size = bytes.len();
    bytes.iter().enumerate()
        .filter(|(k, &v)| bytes[(k + size / 2) % size] == v)
        .map(|(_, &v)| (v - 48) as u32).sum::<u32>()
}

pub fn run(content: &str) {
    println!("{} {}", sum_matching_next(content), sum_matching_half(content));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::sum_matching_next("1122"), 3);
        assert_eq!(super::sum_matching_next("1111"), 4);
        assert_eq!(super::sum_matching_next("1234"), 0);
        assert_eq!(super::sum_matching_next("91212129"), 9);
    }

    #[test]
    fn large() {
        assert_eq!(super::sum_matching_half("1212"), 6);
        assert_eq!(super::sum_matching_half("1221"), 0);
        assert_eq!(super::sum_matching_half("123425"), 4);
        assert_eq!(super::sum_matching_half("123123"), 12);
        assert_eq!(super::sum_matching_half("12131415"), 4);
    }
}
