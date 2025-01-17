use std::collections::HashMap;

type Coord = (i64, i64, i64);

#[derive(Clone, Debug)]
struct Particle {
    index: usize,
    position: Coord,
    velocity: Coord,
    acceleration: Coord,
}

struct Field(Vec<Particle>);

impl Particle {
    fn parse(line: &str, index: usize) -> Self {
        let as_coord = |p, s: &str| {
            let (l, r) = s.split_once('=').unwrap();
            assert_eq!(l, p);
            let a = r.trim_matches(&['<', '>']).split(',').map(|v| {
                v.parse::<i64>().unwrap()
            }).collect::<Vec<_>>();
            (a[0], a[1], a[2])
        };
        let mut parts = line.split(", ");
        let position = as_coord("p", parts.next().unwrap());
        let velocity = as_coord("v", parts.next().unwrap());
        let acceleration = as_coord("a", parts.next().unwrap());
        Self { index, position, velocity, acceleration }
    }

    fn update(&mut self) -> Coord {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
        self.position
    }
}

impl Field {
    fn parse(text: &str) -> Self {
        let data = text.lines().enumerate().map(|(k, v)| {
            Particle::parse(v, k)
        }).collect::<Vec<_>>();
        Self(data)
    }

    fn closest(&self) -> &Particle {
        self.0.iter().min_by_key(|p| (
            distance(&p.acceleration),
            distance(&p.velocity),
            distance(&p.position)
        )).unwrap()
    }

    fn update(&mut self) {
        let mut pos = HashMap::new();
        for i in 0..self.0.len() {
            pos.entry(self.0[i].update())
                .or_insert_with(|| vec![]).push(i);
        }
        let mut del = pos.into_values()
            .filter(|a| a.len() > 1)
            .flat_map(|a| a).collect::<Vec<_>>();
        del.sort();
        for i in del.into_iter().rev() {
            self.0.swap_remove(i);
        }
    }
}

fn distance(coord: &Coord) -> i64 {
    coord.0.abs() + coord.1.abs() + coord.2.abs()
}

pub fn run(content: &str) {
    let mut field = Field::parse(content);
    let init = field.closest().index;
    for _ in 0..1000 { field.update(); }
    println!("{} {}", init, field.0.len());
}
