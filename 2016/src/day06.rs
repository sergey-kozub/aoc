use std::collections::HashMap;

fn dominant(text: &str, idx: usize, rev: bool) -> Option<char> {
    let mut count = HashMap::<char, i32>::new();
    for line in text.lines() {
        if let Some(ch) = line.chars().nth(idx) {
            count.entry(ch).and_modify(|n| *n += 1).or_insert(1);
        }
    }
    count.into_iter()
        .max_by_key(|(_, v)| if rev {-*v} else {*v})
        .map(|(k, _)| k)
}

fn descramble(text: &str, rev: bool) -> String {
    let size = text.split_once("\n").unwrap().0.len();
    (0..size).map(|i| dominant(text, i, rev).unwrap()).collect()
}

pub fn run(content: &str) {
    println!("{} {}", descramble(content, false), descramble(content, true));
}

#[cfg(test)]
mod tests {
    const MESSAGE: &str = "\
        eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\n\
        nssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

    #[test]
    fn small() {
        assert_eq!(super::descramble(MESSAGE, false), "easter");
    }

    #[test]
    fn large() {
        assert_eq!(super::descramble(MESSAGE, true), "advent");
    }
}
