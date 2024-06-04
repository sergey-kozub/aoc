use std::cmp;
use std::fs;
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn split(source: &RangeInclusive<i32>, target: &RangeInclusive<i32>)
    -> Option<Vec<(RangeInclusive<i32>, bool)>> {
    let (ss, se) = (*source.start(), *source.end());
    let (ts, te) = (*target.start(), *target.end());
    if ss <= te && se >= ts {
        let (p1, p2) = (cmp::max(ss, ts), cmp::min(se, te));
        let mut result: Vec<(RangeInclusive<i32>, bool)> = Vec::new();
        if p1 > ss { result.push((ss..=p1-1, false)); }
        result.push((p1..=p2, true));
        if p2 < se { result.push((p2+1..=se, false)); }
        Some(result)
    } else {
        None
    }
}

#[derive(Clone, Debug)]
struct Cube {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
    on: bool,
}

impl Cube {
    fn from(text: &str) -> Cube {
        let p1: Vec<&str> = text.split(' ').collect();
        let p2: Vec<Vec<i32>> = p1[1].split(',').map(|s| {
            s[2..].split("..").map(|v| v.parse::<i32>().unwrap()).collect()
        }).collect();
        Cube {
            x: p2[0][0]..=p2[0][1],
            y: p2[1][0]..=p2[1][1],
            z: p2[2][0]..=p2[2][1],
            on: p1[0] == "on",
        }
    }

    fn size(&self) -> usize {
        (self.x.end() - self.x.start() + 1) as usize *
        (self.y.end() - self.y.start() + 1) as usize *
        (self.z.end() - self.z.start() + 1) as usize
    }

    fn split(&self, other: &Cube) -> Option<Vec<Cube>> {
        assert!(self.on);
        let mut result: Vec<Cube> = Vec::new();
        for (xr, xf) in split(&self.x, &other.x)? {
            for (yr, yf) in split(&self.y, &other.y)? {
                for (zr, zf) in split(&self.z, &other.z)? {
                    let intersects = xf && yf && zf;
                    if !intersects {
                        result.push(Cube {
                            x: xr.clone(),
                            y: yr.clone(),
                            z: zr.clone(),
                            on: true,
                        });
                    }
                }
            }
        }
        Some(result)
    }

    fn merge(&self, other: &Cube) -> Option<Vec<Cube>> {
        assert!(self.on && other.on);
        let mut r1 = self.split(other)?;
        let mut r2 = other.split(self)?;
        if r1.len() < r2.len() {
            r1.push(other.clone());
            Some(r1)
        } else {
            r2.push(self.clone());
            Some(r2)
        }
    }
}

fn simple(cubes: &[Cube]) -> usize {
    let mut track: HashSet<(i32, i32, i32)> = HashSet::new();
    for cube in cubes {
        if cube.size() > 1_000_000 { continue; }
        for x in cube.x.clone() { for y in cube.y.clone() { for z in cube.z.clone() {
            if cube.on {
                track.insert((x, y, z));
            } else {
                track.remove(&(x, y, z));
            }
        }}}
    }
    track.len()
}

fn complete(cubes: &[Cube]) -> usize {
    let mut result: Vec<Cube> = Vec::new();
    for cube in cubes {
        if cube.on {
            let mut adding = vec![cube.clone()];
            while let Some(next) = adding.pop() {
                let merge = result.iter().enumerate().find_map(|(index, cube)| {
                    let more = next.merge(cube)?;
                    Some((index, more))
                });
                match merge {
                    Some((index, mut cubes)) => {
                        result.swap_remove(index);
                        adding.append(&mut cubes);
                    },
                    None => result.push(next),
                }
            }
        } else {
            let mut mods: Vec<Cube> = Vec::new();
            for next in result.into_iter() {
                match next.split(cube) {
                    Some(mut more) => mods.append(&mut more),
                    None => mods.push(next),
                }
            }
            result = mods;
        }
    }
    result.iter().map(|c| c.size()).sum()
}

fn main() {
    let input: Vec<Cube> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(Cube::from).collect();
    println!("{} {}", simple(&input), complete(&input))
}
