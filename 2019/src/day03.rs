use std::collections::HashMap;

type Point = (i32, i32);
type Path = HashMap<Point, usize>;

fn build_path(text: &str) -> Path {
    let mut result: Path = HashMap::new();
    let mut pos: Point = (0, 0);
    let mut count: usize = 0;

    for step in text.split(',') {
        let n = step[1..].parse::<i32>().unwrap();
        let (mut dx, mut dy): (i32, i32) = (0, 0);
        match step.as_bytes()[0] {
            b'U' => dy = 1,
            b'R' => dx = 1,
            b'D' => dy = -1,
            b'L' => dx = -1,
            _ => panic!("unknown direction"),
        }
        for _ in 0..n {
            pos = (pos.0 + dx, pos.1 + dy);
            count += 1;
            result.entry(pos).or_insert(count);
        }
    }
    result
}

fn solve_1(path1: &Path, path2: &Path) -> i32 {
    path1.keys().map(|p| {
        if path2.contains_key(p) {p.0.abs() + p.1.abs()}
            else {i32::MAX}
    }).min().unwrap()
}

fn solve_2(path1: &Path, path2: &Path) -> usize {
    path1.iter().map(|(k, v)| {
        if let Some(value) = path2.get(k) {v + value}
            else {usize::MAX}
    }).min().unwrap()
}

pub fn run(content: &str) {
    let paths: Vec<Path> = content.lines().map(build_path).collect();
    let dist_1 = solve_1(&paths[0], &paths[1]);
    let dist_2 = solve_2(&paths[0], &paths[1]);
    println!("{} {}", dist_1, dist_2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let run = |s1, s2| super::solve_1(
            &super::build_path(s1), &super::build_path(s2));
        assert_eq!(6,
            run("R8,U5,L5,D3", "U7,R6,D4,L4"));
        assert_eq!(159,
            run("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"));
        assert_eq!(135,
            run("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
    }

    #[test]
    fn part2() {
        let run = |s1, s2| super::solve_2(
            &super::build_path(s1), &super::build_path(s2));
        assert_eq!(30,
            run("R8,U5,L5,D3", "U7,R6,D4,L4"));
        assert_eq!(610,
            run("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"));
        assert_eq!(410,
            run("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
    }
}
