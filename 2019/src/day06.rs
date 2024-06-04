use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

struct Orbits(HashMap<String, String>);

impl Orbits {
    fn parse(text: &str) -> Orbits {
        Orbits(text.lines().map(|s| {
            let a: Vec<&str> = s.split(')').collect();
            (String::from(a[1]), String::from(a[0]))
        }).collect())
    }

    fn count_total(&self, key: &str, memo: &mut HashMap<String, usize>) -> usize {
        if let Some(total) = memo.get(key) {
            return *total;
        }
        if let Some(other) = self.0.get(key) {
            let result = self.count_total(other, memo) + 1;
            memo.insert(String::from(key), result);
            return result;
        }
        0
    }

    fn count_all(&self) -> usize {
        let mut memo: HashMap<String, usize> = HashMap::new();
        self.0.keys().map(|k| self.count_total(k, &mut memo)).sum()
    }

    fn get_path(&self, key: &str) -> Vec<String> {
        if let Some(other) = self.0.get(key) {
            let mut result = self.get_path(other);
            result.push(String::from(other));
            return result;
        }
        Vec::new()
    }

    fn shortest_path(&self, from: &str, to: &str) -> usize {
        let path_1: HashSet<String> = HashSet::from_iter(self.get_path(from).into_iter());
        let path_2: HashSet<String> = HashSet::from_iter(self.get_path(to).into_iter());
        path_1.symmetric_difference(&path_2).count()
    }
}

pub fn run(content: &str) {
    let orbits = Orbits::parse(content);
    println!("{} {}", orbits.count_all(), orbits.shortest_path("YOU", "SAN"))
}

#[cfg(test)]
mod tests {
    use super::Orbits;
    const INPUT: &str =
        "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";

    #[test]
    fn part1() {
        let orbits = Orbits::parse(INPUT);
        assert_eq!(orbits.count_all(), 42);
    }

    #[test]
    fn part2() {
        let mut input_ext = String::from(INPUT);
        input_ext.push_str("K)YOU\nI)SAN\n");
        let orbits = Orbits::parse(&input_ext);
        assert_eq!(orbits.shortest_path("YOU", "SAN"), 4);
    }
}
