
type Point = (i32, i32);

#[derive(Clone, Copy, Debug)]
enum Direction { Up, Right, Down, Left }

#[derive(Clone, Copy, Debug, PartialEq)]
enum Turn { Left, Right }

impl Turn {
    fn flip(&self) -> Self {
        match self {
            Turn::Left => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

fn parse(text: &str) -> Vec<Point> {
    text.lines().map(|line| {
        let (l, r) = line.split_once(',').unwrap();
        let x = l.parse::<i32>().unwrap();
        let y = r.parse::<i32>().unwrap();
        (x, y)
    }).collect()
}

fn find_simple(data: &[Point]) -> u64 {
    (0..data.len()).flat_map(|i| {
        let (x1, y1) = data[i];
        (i + 1..data.len()).map(move |j| {
            let (x2, y2) = data[j];
            let dx = (x2 - x1).abs() + 1;
            let dy = (y2 - y1).abs() + 1;
            (dx as u64) * (dy as u64)
        })
    }).max().unwrap()
}

fn overlaps(a: (Point, Point), b: (Point, Point)) -> bool {
    let ((ax1, ay1), (ax2, ay2)) = a;
    let ((bx1, by1), (bx2, by2)) = b;
    let intersects = |s: Point, t: Point| s.0.max(t.0) <= s.1.min(t.1);
    intersects((ax1, ax2), (bx1, bx2)) && intersects((ay1, ay2), (by1, by2))
}

fn remove_all<T>(data: &mut Vec<T>, mut idx: Vec<usize>) {
    idx.sort();
    while let Some(i) = idx.pop() {
        data.remove(i);
    }
}

fn find_green(data: &[Point]) -> u64 {
    let mut copy = data.to_vec();
    copy.push(copy[0]);

    let mut dirs = copy.windows(2).map(|a| {
        let (x1, y1) = a[0];
        let (x2, y2) = a[1];
        if x1 == x2 && y2 < y1 {Direction::Up}
        else if y1 == y2 && x2 > x1 {Direction::Right}
        else if x1 == x2 && y2 > y1 {Direction::Down}
        else if y1 == y2 && x2 < x1 {Direction::Left}
        else {panic!()}
    }).collect::<Vec<_>>();
    dirs.push(dirs[0]);

    let mut turns = dirs.windows(2).map(|a| match a {
        [Direction::Right, Direction::Down] |
        [Direction::Down, Direction::Left] |
        [Direction::Left, Direction::Up] |
        [Direction::Up, Direction::Right] => Turn::Right,
        [Direction::Right, Direction::Up] |
        [Direction::Up, Direction::Left] |
        [Direction::Left, Direction::Down] |
        [Direction::Down, Direction::Right] => Turn::Left,
        _ => panic!(),
    }).collect::<Vec<_>>();

    let count_left = turns.iter().filter(|&&x| x == Turn::Left).count();
    let count_right = turns.len() - count_left;
    let turn = if count_left > count_right {
        assert_eq!(count_left - count_right, 4);
        Turn::Left
    } else {
        assert_eq!(count_right - count_left, 4);
        Turn::Right
    };

    let mut n = turns.len();
    let mut exclude = vec![];
    while n > 4 {
        let i = (0..n).filter(|&i| {
            turns[i] != turn && turns[(i + 1) % n] == turn &&
            (turns[(i + n - 1) % n] == turn || turns[(i + n - 2) % n] == turn)
        }).next().unwrap();
        let p = (0..3).map(|j| copy[(i + j) % n]).collect::<Vec<_>>();
        let (x1, x2) = (p[0].0.min(p[2].0), p[0].0.max(p[2].0));
        let (y1, y2) = (p[0].1.min(p[2].1), p[0].1.max(p[2].1));
        let inside = |(x, y): Point| x >= x1 && x <= x2 && y >= y1 && y <= y2;
        let prev = copy[(i + n - 1) % n];

        let c = vec![(x1, y1), (x2, y1), (x2, y2), (x1, y2)];
        let m = c.iter().position(|&t| !p.iter().any(|&s| s == t)).unwrap();
        assert!(prev != c[m]);
        copy[i] = c[m];
        if inside(prev) {
            turns[(i + n - 2) % n] = turn.flip();
            turns[(i + n - 1) % n] = turn;
        }
        remove_all(&mut copy, vec![(i + 1) % n, (i + 2) % n]);
        remove_all(&mut turns, vec![i, (i + 1) % n]);
        n -= 2;

        let d = if inside(prev) {1} else {0};
        let area = match m {
            0 => ((x1 + d, y1 + d), (x2 - 1, y2 - 1)),
            1 => ((x1 + 1, y1 + d), (x2 - d, y2 - 1)),
            2 => ((x1 + 1, y1 + 1), (x2 - d, y2 - d)),
            3 => ((x1 + d, y1 + 1), (x2 - 1, y2 - d)),
            _ => panic!(),
        };
        exclude.push(area);
    }

    (0..data.len()).flat_map(|i| {
        let (x1, y1) = data[i];
        let copy = exclude.clone();
        (i + 1..data.len()).filter_map(move |j| {
            let (x2, y2) = data[j];
            let (xmin, xmax) = (x1.min(x2), x1.max(x2));
            let (ymin, ymax) = (y1.min(y2), y1.max(y2));
            let area = ((xmin, ymin), (xmax, ymax));
            if copy.iter().any(|&t| overlaps(t, area)) {
                return None;
            }
            let dx = (xmax - xmin + 1) as u64;
            let dy = (ymax - ymin + 1) as u64;
            Some(dx * dy)
        })
    }).max().unwrap()
}

pub fn run(content: &str) {
    let data = parse(content);
    println!("{} {}", find_simple(&data), find_green(&data));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn small() {
        assert_eq!(super::find_simple(&super::parse(TEST)), 50);
    }

    #[test]
    fn large() {
        assert_eq!(super::find_green(&super::parse(TEST)), 24);
    }
}
