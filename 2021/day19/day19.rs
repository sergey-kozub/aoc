use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;

type Point = (i32, i32, i32);

fn permute(p: &Point, n: u8) -> Point {
    match n {
        0 => (p.0, p.1, p.2),
        1 => (p.0, p.2, p.1),
        2 => (p.1, p.0, p.2),
        3 => (p.1, p.2, p.0),
        4 => (p.2, p.0, p.1),
        5 => (p.2, p.1, p.0),
        _ => unreachable!(),
    }
}

fn flip(p: &Point, n: u8) -> Point {
    (if n & 1 > 0 { -p.0 } else { p.0 },
     if n & 2 > 0 { -p.1 } else { p.1 },
     if n & 4 > 0 { -p.2 } else { p.2 })
}

fn mutate(p: &Point, n: u8) -> Point {
    flip(&permute(p, n >> 3), n & 7)
}

fn diff(a: &Point, b: &Point) -> Point {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

#[derive(Debug)]
struct Scanner {
    number: u32,
    beacons: Vec<Point>,
    offsets: HashMap<Point, Vec<(usize, usize)>>,
    position: Point,
}

impl Scanner {
    fn parse(text: &str, number: u32) -> Scanner {
        let beacons: Vec<Point> = text.lines().skip(1).map(|line| {
            let a: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            (a[0], a[1], a[2])
        }).collect();
        let mut offsets: HashMap<Point, Vec<(usize, usize)>> = HashMap::new();
        for i in 0..beacons.len() {
            for j in i+1..beacons.len() {
                let key = diff(&beacons[i], &beacons[j]);
                offsets.entry(key).or_default().push((i, j));
            }
        }
        Scanner { number, beacons, offsets, position: (0, 0, 0) }
    }

    fn get_mapping(&self, other: &Scanner, variant: u8) -> HashMap<usize, usize> {
        let mut possible: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (key, value) in &other.offsets {
            let point = mutate(key, variant);
            if let Some(vref) = self.offsets.get(&point) {
                for (i1, j1) in vref {
                    for (i2, j2) in value {
                        possible.entry(*i1).or_default().insert(*i2);
                        possible.entry(*j1).or_default().insert(*j2);
                    }
                }
            }
        }
        let mut definite: HashMap<usize, usize> = HashMap::new();
        for (key, value) in possible {
            if value.len() == 1 {
                definite.insert(key, *value.iter().next().unwrap());
            }
        }
        definite
    }

    fn find_match(&self, other: &Scanner, min_size: usize) -> Option<Scanner> {
        let options: Vec<u8> = (0..48).filter(
            |v| self.get_mapping(other, *v).len() >= min_size).collect();
        if options.len() != 1 { return None; }

        let variant = options[0];
        let mut result = Scanner {
            number: other.number,
            beacons: other.beacons.iter().map(
                |p| mutate(p, variant)).collect(),
            offsets: other.offsets.iter().map(
                |(k, v)| (mutate(k, variant), v.clone())).collect(),
            position: (0, 0, 0),
        };

        let deltas: Vec<Point> = self.get_mapping(other, variant)
            .into_iter().map(|(i, j)| {
                diff(&self.beacons[i], &result.beacons[j])
            }).collect();
        assert!(deltas.iter().all(|&x| x == deltas[0]), "incorrect deltas");

        let rel = deltas[0];
        for pos in result.beacons.iter_mut() {
            *pos = (pos.0 + rel.0, pos.1 + rel.1, pos.2 + rel.2);
        }
        result.position = rel;
        Some(result)
    }

    fn match_all(mut input: Vec<Scanner>, sizes: RangeInclusive<usize>) -> Vec<Scanner> {
        let mut result = vec![input.pop().unwrap()];
        let mut try_sizes: Vec<usize> = sizes.collect();
        let mut size = try_sizes.pop().unwrap();

        'outer: while !input.is_empty() {
            for i in (0..result.len()).rev() {
                for j in 0..input.len() {
                    if let Some(scanner) = result[i].find_match(&input[j], size) {
                        result.push(scanner);
                        input.remove(j);
                        continue 'outer;
                    }
                }
            }
            println!("Reducing size {} (scanners left: {})", size, input.len());
            match try_sizes.pop() {
                Some(n) => size = n,
                None => panic!("giving up"),
            }
        }
        result
    }
}

fn main() {
    let input: Vec<Scanner> = fs::read_to_string("input.txt").expect("Error reading input")
        .split("\n\n").enumerate().map(|(i, s)| Scanner::parse(s, i as u32)).collect();

    let result = Scanner::match_all(input, 11..=12);
    let mut points: HashSet<Point> = HashSet::new();
    let mut max_dist: i32 = 0;
    for scanner in &result {
        for pt in &scanner.beacons {
            points.insert(*pt);
        }
        for other in &result {
            let d = diff(&scanner.position, &other.position);
            let dist = d.0.abs() + d.1.abs() + d.2.abs();
            if dist > max_dist { max_dist = dist; }
        }
    }
    println!("{} {}", points.len(), max_dist);
}
