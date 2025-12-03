use md5;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug)]
struct Search {
    salt: String,
    stretch: usize,
    index: usize,
    limit: usize,
    found: HashSet<(usize, u8)>,
    valid: BinaryHeap<(isize, u8)>,
}

fn find_triplet(hash: &[u8]) -> Option<u8> {
    (0..hash.len() - 2).filter(|&i| {
        hash[i + 1] == hash[i] && hash[i + 2] == hash[i]
    }).next().map(|i| hash[i])
}

fn calc_hash(input: &str) -> Vec<u8> {
    md5::compute(input.as_bytes()).0.iter().flat_map(|x| {
        [x >> 4, x & 0xf]
    }).collect::<Vec<_>>()
}

impl Search {
    fn new(salt: String, stretch: usize) -> Self {
        Self {
            salt, stretch,
            index: 0, limit: 1000,
            found: HashSet::new(),
            valid: BinaryHeap::new(),
        }
    }
}

impl Iterator for Search {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            let mut hash = calc_hash(&format!("{}{}", self.salt, self.index));
            for _ in 0..self.stretch {
                let temp = hash.iter()
                    .map(|&v| char::from_digit(v as u32, 16).unwrap())
                    .collect::<String>();
                hash = calc_hash(&temp);
            }
            let threshold = self.index as isize - self.limit as isize;
            (0_u8..16).filter_map(|i| {
                let needle = [i; 5];
                hash.windows(5).position(|w| w == &needle).map(|_| i)
            }).for_each(|digit| {
                self.found.extract_if(|x| x.1 == digit).for_each(|x| {
                    if x.0 as isize >= threshold {
                        self.valid.push((-(x.0 as isize), x.1));
                    }
                });
            });
            if let Some(digit) = find_triplet(&hash) {
                self.found.insert((self.index, digit));
            }
            self.index += 1;
            if let Some(top) = self.valid.peek() {
                if -top.0 == threshold {
                    let item = self.valid.pop().unwrap();
                    return Some(-item.0 as usize);
                }
            }
        }
    }
}

pub fn run(content: &str) {
    let res1 = Search::new(content.into(), 0).skip(63).next().unwrap();
    let res2 = Search::new(content.into(), 2016).skip(63).next().unwrap();
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let mut iter = super::Search::new("abc".into(), 0);
        assert_eq!(iter.next(), Some(39));
        assert_eq!(iter.next(), Some(92));
        assert_eq!(iter.skip(61).next(), Some(22728));
    }

    #[test]
    fn large() {
        // let mut iter = super::Search::new("abc".into(), 2016);
        // assert_eq!(iter.next(), Some(10));
    }
}
