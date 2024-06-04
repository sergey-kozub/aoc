use itertools::Itertools;
use std::cmp::{Ordering, min};
use std::iter::Peekable;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    Value(u32),
    List(Vec<Item>),
}

impl Item {
    fn from(input: &str) -> Item {
        Item::from_iter(&mut input.chars().peekable())
    }

    fn from_iter<I: Iterator<Item=char>>(chars: &mut Peekable<I>) -> Item {
        match chars.next().unwrap() {
            c @ '0'..='9' => {
                let mut value = c.to_digit(10).unwrap();
                while let Some(c) = chars.next_if(|x| x.is_ascii_digit()) {
                    value = value * 10 + c.to_digit(10).unwrap();
                }
                Item::Value(value)
            },
            '[' => {
                let mut items = Vec::<Item>::new();
                if matches!(chars.next_if_eq(&']'), None) { loop {
                    items.push(Item::from_iter(chars));
                    match chars.next().unwrap() {
                        ']' => break,
                        ',' => (),
                        _ => panic!("Incorrect input")
                    }
                }}
                Item::List(items)
            },
            _ => panic!("Incorrect input")
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        match self {
            Item::Value(a) => match other {
                Item::Value(b) => a.cmp(b),
                Item::List(_) => other.cmp(self).reverse(),
            },
            Item::List(a) => match other {
                Item::Value(b) => {
                    self.cmp(&Item::List(vec![Item::Value(*b)]))
                },
                Item::List(b) => {
                    for i in 0..min(a.len(), b.len()) {
                        let res = a[i].cmp(&b[i]);
                        if res != Ordering::Equal { return res; }
                    }
                    a.len().cmp(&b.len())
                },
            },
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(content: &str) {
    let pairs: Vec<(Item, Item)> = content.split("\n\n").map(|s|
        s.lines().map(Item::from).collect_tuple().unwrap()
    ).collect();
    let score: usize = pairs.iter().enumerate().filter_map(|(k, (a, b))|
        if a < b { Some(k + 1) } else { None }
    ).sum();

    let mut items: Vec<Item> = pairs.into_iter().flat_map(|(a, b)| [a, b]).collect();
    let divider = vec![Item::from("[[2]]"), Item::from("[[6]]")];
    items.append(&mut divider.clone());
    items.sort();
    let pos: Vec<usize> = items.iter().enumerate().filter_map(|(k, v)|
        if divider.contains(v) { Some(k + 1) } else { None }
    ).collect();

    println!("{} {}", score, pos[0] * pos[1]);
}

#[cfg(test)]
mod tests {
    fn check(s1: &str, s2: &str, less: bool) {
        let t1 = super::Item::from(s1);
        let t2 = super::Item::from(s2);
        assert_eq!(t1 < t2, less, "{s1} {s2}");
    }

    #[test]
    pub fn compare() {
        check("[1,1,3,1,1]", "[1,1,5,1,1]", true);
        check("[[1],[2,3,4]]", "[[1],4]", true);
        check("[9]", "[[8,7,6]]", false);
        check("[[4,4],4,4]", "[[4,4],4,4,4]", true);
        check("[7,7,7,7]", "[7,7,7]", false);
        check("[]", "[3]", true);
        check("[[[]]]", "[[]]", false);
        check("[1,[2,[3,[4,[5,6,7]]]],8,9]",
              "[1,[2,[3,[4,[5,6,0]]]],8,9]", false);
    }
}
