use std::collections::HashSet;

fn search(s: &str, n: usize) -> usize {
    s.as_bytes().windows(n).enumerate().take_while(|(_, v)| {
        HashSet::<u8>::from_iter(v.to_vec()).len() < n
    }).last().unwrap().0 + n + 1
}

pub fn run(content: &str) {
    let clean = content.trim_end();
    println!("{} {}", search(clean, 4), search(clean, 14));
}

#[cfg(test)]
mod tests {
    fn check(s: &str, r4: usize, r14: usize) {
        assert_eq!(super::search(s, 4), r4);
        assert_eq!(super::search(s, 14), r14);
    }

    #[test]
    fn search() {
        check("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19);
        check("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23);
        check("nppdvjthqldpwncqszvftbrmjlhg", 6, 23);
        check("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29);
        check("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26);
    }
}
