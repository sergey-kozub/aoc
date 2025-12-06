use md5;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Path(String);

impl Path {
    fn get_open_doors(&self) -> Vec<char> {
        let is_open = |c| matches!(c, 'b'|'c'|'d'|'e'|'f');
        let hash = format!("{:x}", md5::compute(self.0.as_bytes()));
        let mut iter = hash.chars();
        let mut result = vec![];
        if is_open(iter.next().unwrap()) { result.push('U'); }
        if is_open(iter.next().unwrap()) { result.push('D'); }
        if is_open(iter.next().unwrap()) { result.push('L'); }
        if is_open(iter.next().unwrap()) { result.push('R'); }
        result
    }
}

fn navigate(salt: String, long: bool) -> String {
    const EDGE: i32 = 3;
    let mut queue = VecDeque::from([(0_i32, 0_i32, Path(salt.clone()))]);
    let mut longest = String::new();
    while let Some((x, y, mut path)) = queue.pop_front() {
        if x == EDGE && y == EDGE {
            longest = path.0.split_off(salt.len());
            if !long { break; }
            else { continue; }
        }
        for door in path.get_open_doors() {
            let next = match door {
                'U' if y > 0 => Some((x, y - 1)),
                'D' if y < EDGE => Some((x, y + 1)),
                'L' if x > 0 => Some((x - 1, y)),
                'R' if x < EDGE => Some((x + 1, y)),
                _ => None,
            };
            if let Some((nx, ny)) = next {
                let np = path.0.clone() + &door.to_string();
                queue.push_back((nx, ny, Path(np)));
            }
        }
    }
    longest
}

pub fn run(content: &str) {
    let res1 = navigate(content.into(), false);
    let res2 = navigate(content.into(), true).len();
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::navigate("ihgpwlah".into(), false), "DDRRRD");
        assert_eq!(super::navigate("kglvqrro".into(), false), "DDUDRLRRUDRD");
        assert_eq!(super::navigate("ulqzkmiv".into(), false),
                   "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }
}
