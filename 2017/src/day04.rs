use std::collections::HashSet;

fn is_valid(text: &str, sort: bool) -> bool {
    let iter = || text.split_whitespace().map(|s| {
        let mut a = s.to_owned().into_bytes();
        if sort { a.sort(); }
        a
    });
    let words = iter().collect::<HashSet<_>>();
    iter().count() == words.len()
}

pub fn run(content: &str) {
    let valid = |v| content.lines().filter(|&s| is_valid(s, v)).count();
    println!("{} {}", valid(false), valid(true));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::is_valid("aa bb cc dd ee", false), true);
        assert_eq!(super::is_valid("aa bb cc dd aa", false), false);
    }

    #[test]
    fn large() {
        assert_eq!(super::is_valid("a ab abc abd abf abj", true), true);
        assert_eq!(super::is_valid("abcde xyz ecdab", true), false);
    }
}
