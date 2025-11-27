use md5;

struct PassIter {
    head: String,
    index: usize,
}

impl Iterator for PassIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let s = format!("{}{}", self.head, self.index);
        self.index += 1;
        let d = md5::compute(s.as_bytes());
        Some(format!("{:x}", d))
    }
}

fn find_pass_1(head: &str, count: usize) -> String {
    PassIter { head: head.into(), index: 0 }
        .filter_map(|d| if &d[..5] == "00000" {d.chars().nth(5)} else {None})
        .take(count).collect()
}

fn find_pass_2(head: &str, count: usize) -> String {
    let mut result = vec!['?'; count];
    PassIter { head: head.into(), index: 0 }
        .filter_map(|d| if &d[..5] == "00000" {
            let pos = d.chars().nth(5).unwrap().to_digit(16).unwrap() as usize;
            let val = d.chars().nth(6).unwrap();
            if pos < count {Some((pos, val))} else {None}
        } else {None})
        .filter_map(|(k, v)| if result[k] == '?' {
            result[k] = v;
            Some(v)
        } else {None})
        .take(count).for_each(|_| {});
    result.into_iter().collect()
}

pub fn run(content: &str) {
    println!("{}", find_pass_1(content, 8));
    println!("{}", find_pass_2(content, 8));
}
