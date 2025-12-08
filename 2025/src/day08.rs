use std::collections::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point(i64, i64, i64);

#[derive(Clone, Debug)]
struct Boxes(Vec<Point>);

impl Point {
    fn parse(text: &str) -> Self {
        let a = text.split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Self(a[0], a[1], a[2])
    }

    fn distance(&self, other: &Point) -> f64 {
        (((self.0 - other.0).pow(2) +
          (self.1 - other.1).pow(2) +
          (self.2 - other.2).pow(2)) as f64).sqrt()
    }
}

impl Boxes {
    fn parse(text: &str) -> Self {
        let a = text.lines().map(Point::parse).collect::<Vec<_>>();
        Self(a)
    }

    fn pick(&self, count: usize) -> usize {
        let n = self.0.len();
        let mut pairs = (0..n).flat_map(|i| (i + 1..n).map(move |j| {
            let (a, b) = (self.0[i].clone(), self.0[j].clone());
            (a.distance(&b), a, b)
        })).collect::<Vec<_>>();
        pairs.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        pairs.truncate(count);

        let mut connect = HashMap::<Point, usize>::new();
        let mut circuit = 0;
        let mut last = None;
        for (_, a, b) in pairs {
            let new = if let Some(&n) = connect.get(&a) {
                if let Some(&m) = connect.get(&b) {
                    if m != n {
                        for value in connect.values_mut() {
                            if *value == m { *value = n; }
                        }
                        true
                    } else {false}
                } else {
                    connect.insert(b.clone(), n);
                    true
                }
            } else {
                if let Some(&m) = connect.get(&b) {
                    connect.insert(a.clone(), m);
                    true
                } else {
                    circuit += 1;
                    connect.insert(a.clone(), circuit);
                    connect.insert(b.clone(), circuit);
                    true
                }
            };
            if new {
                last = Some((a, b));
            }
        }

        if count != usize::MAX {
            let mut count = HashMap::new();
            for n in connect.values() {
                count.entry(n).and_modify(|c| *c += 1).or_insert(1);
            }
            let mut a = count.values().collect::<Vec<_>>();
            a.sort_unstable_by(|a, b| b.cmp(&a));
            a[0] * a[1] * a[2]
        } else {
            last.map(|(a, b)| a.0 * b.0).unwrap() as usize
        }
    }
}

pub fn run(content: &str) {
    let boxes = Boxes::parse(content);
    println!("{} {}", boxes.pick(1000), boxes.pick(usize::MAX));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn small() {
        assert_eq!(super::Boxes::parse(TEST).pick(10), 40);
    }

    #[test]
    fn large() {
        assert_eq!(super::Boxes::parse(TEST).pick(usize::MAX), 25272);
    }
}
