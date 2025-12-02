use std::ops::RangeInclusive;

fn is_invalid(value: usize, simple: bool) -> bool {
    let s = value.to_string();
    let m = s.len() / 2;
    if simple {
        return m * 2 == s.len() && s[..m].repeat(2) == s;
    }
    for n in 1..=m {
        if s.len() % n != 0 { continue; }
        let t = s[..n].repeat(s.len() / n);
        if s == t { return true; }
    }
    false
}

fn sum_invalid(text: &str, simple: bool) -> usize {
    parse(text).into_iter()
        .flat_map(|r| r.filter(|&x| is_invalid(x, simple)))
        .sum::<usize>()
}

fn parse(text: &str) -> Vec<RangeInclusive<usize>> {
    let parse = |x: &str| x.parse::<usize>().unwrap();
    text.split(',').map(|s| {
        let (l, r) = s.split_once('-').unwrap();
        parse(l)..=parse(r)
    }).collect()
}

pub fn run(content: &str) {
    let res1 = sum_invalid(content, true);
    let res2 = sum_invalid(content, false);
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn small() {
        assert_eq!(super::sum_invalid(TEST, true), 1227775554);
    }

    #[test]
    fn large() {
        assert_eq!(super::sum_invalid(TEST, false), 4174379265);
    }
}
