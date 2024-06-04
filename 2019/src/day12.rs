use num::integer::lcm;
use regex::Regex;

type Coord = [i32; 3];

#[derive(Clone, Debug)]
struct Moon {
    position: Coord,
    velocity: Coord,
}

impl Moon {
    fn from(text: &str) -> Moon {
        let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        let m = re.captures(text).unwrap();
        let a: Vec<i32> = (1..=3).map(|i| m.get(i).unwrap()
            .as_str().parse::<i32>().unwrap()).collect();
        Moon { position: [a[0], a[1], a[2]], velocity: [0, 0, 0] }
    }
}

#[derive(Clone, Debug)]
struct Moons(Vec<Moon>);

impl Moons {
    fn from(text: &str) -> Moons {
        Moons(text.lines().map(Moon::from).collect())
    }

    fn gravity(&mut self) {
        for i in 0..3 {
            let d: Vec<i32> = self.0.iter().map(|a| {
                self.0.iter().map(|b| {
                    let c = b.position[i] - a.position[i];
                    if c > 0 { 1 } else if c < 0 { -1 } else { 0 }
                }).sum()
            }).collect();
            for j in 0..d.len() {
                let m = &mut self.0[j];
                let v = m.velocity[i] + d[j];
                m.velocity[i] = v;
                m.position[i] += v;
            }
        }
    }

    fn process(&self, steps: usize) -> i32 {
        let mut current = (*self).clone();
        for _ in 0..steps { current.gravity(); }
        current.0.iter().map(|m| {
            (0..3).map(|i| m.position[i].abs()).sum::<i32>() *
            (0..3).map(|i| m.velocity[i].abs()).sum::<i32>()
        }).sum::<i32>()
    }

    fn axis(&self, index: usize) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for m in &self.0 {
            result.push(m.position[index]);
            result.push(m.velocity[index]);
        }
        result
    }

    fn search(&self, index: usize) -> usize {
        let initial = self.axis(index);
        let mut current = (*self).clone();
        let mut steps: usize = 0;
        loop {
            steps += 1;
            current.gravity();
            if current.axis(index) == initial { break; }
        }
        steps
    }

    fn repeats(&self) -> usize {
        lcm(lcm(self.search(0), self.search(1)), self.search(2))
    }
}

pub fn run(content: &str) {
    let moons = Moons::from(content);
    println!("{} {}", moons.process(1000), moons.repeats());
}

#[cfg(test)]
mod tests {
    fn example1() -> super::Moons {
        super::Moons::from("\
            <x=-1, y=0, z=2>\n\
            <x=2, y=-10, z=-7>\n\
            <x=4, y=-8, z=8>\n\
            <x=3, y=5, z=-1>")
    }

    fn example2() -> super::Moons {
        super::Moons::from("\
            <x=-8, y=-10, z=0>\n\
            <x=5, y=5, z=10>\n\
            <x=2, y=-7, z=3>\n\
            <x=9, y=-8, z=-3>")
    }

    #[test]
    fn part1() {
        assert_eq!(179, example1().process(10));
        assert_eq!(1940, example2().process(100));
    }

    #[test]
    fn part2() {
        assert_eq!(2772, example1().repeats());
        assert_eq!(4686774924, example2().repeats());
    }
}
