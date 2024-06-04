use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;

#[derive(Debug)]
struct Segments {
    digits: Vec<String>,
    display: Vec<String>,
}

impl Segments {
    fn from(text: &str) -> Segments {
        let mut parts = text.split(" | ")
            .map(|s| s.split_whitespace().map(String::from).collect());
        Segments {
            digits: parts.next().unwrap(),
            display: parts.next().unwrap(),
        }
    }
    
    fn find<F>(&self, n: usize, f: F) -> HashSet<char>
        where F: Fn(&HashSet<char>) -> bool {
        let mut matches: Vec<HashSet<char>> = self.digits.iter()
            .filter(|s| s.len() == n)
            .map(|s| HashSet::from_iter(s.chars()))
            .filter(f).collect();
        assert_eq!(matches.len(), 1, "size: {}", n);
        matches.pop().unwrap()
    }

    fn decode(&self) -> u32 {
        let d1 = self.find(2, |_| true);
        let d7 = self.find(3, |_| true);
        let d4 = self.find(4, |_| true);
        let d8 = self.find(7, |_| true);
        let d2 = self.find(5, |x| x.difference(&d4).count() == 3);
        let d3 = self.find(5, |x| x.difference(&d7).count() == 2);
        let d5 = self.find(5, |x| x != &d2 && x != &d3);
        let d0 = self.find(6, |x| x.difference(&d5).count() == 2);
        let d9 = self.find(6, |x| x.difference(&d3).count() == 1);
        let d6 = self.find(6, |x| x != &d0 && x != &d9);
        let dset = vec!(d0, d1, d2, d3, d4, d5, d6, d7, d8, d9);
        let result = self.display.iter()
            .map(|s| HashSet::from_iter(s.chars()))
            .map(|x| dset.iter().position(|d| d == &x).unwrap() as u32);
        result.rev().enumerate().map(|(i, v)| v * 10_u32.pow(i as u32)).sum()
    }
}

fn main() {
    let input: Vec<Segments> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(Segments::from).collect();

    let simple: usize = input.iter().map(
        |x| x.display.iter().filter(|s| s.len() <= 4 || s.len() == 7).count()).sum();
    let decoded: u32 = input.iter().map(|x| x.decode()).sum();
    println!("{} {}", simple, decoded)
}
