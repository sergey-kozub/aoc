use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Axis {X, Y}

type Fold = (Axis, i32);
type Point = (i32, i32);

fn fold_x(points: HashSet<Point>, x: i32) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();
    for pt in points {
        let new_x = if pt.0 > x { x * 2 - pt.0 } else { pt.0 };
        result.insert((new_x, pt.1));
    }
    result
}

fn fold_y(points: HashSet<Point>, y: i32) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();
    for pt in points {
        let new_y = if pt.1 > y { y * 2 - pt.1 } else { pt.1 };
        result.insert((pt.0, new_y));
    }
    result
}

fn fold_all(points: HashSet<Point>, folds: Vec<Fold>) -> HashSet<Point> {
    let mut result = points.clone();
    for fold in folds {
        result = match fold {
            (Axis::X, x) => fold_x(result, x),
            (Axis::Y, y) => fold_y(result, y),
        };
        // println!("{}", result.len());
    }
    result
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").expect("Error reading input")
        .split("\n\n").map(String::from).collect();

    let mut points: HashSet<Point> = HashSet::new();
    for line in input[0].lines() {
        let parts: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        points.insert((parts[0], parts[1]));
    }
    let mut folds: Vec<Fold> = Vec::new();
    for line in input[1].lines() {
        let parts: Vec<&str> = line.split('=').collect();
        folds.push((match parts[0].chars().last().unwrap() {
            'x' => Axis::X,
            'y' => Axis::Y,
            _ => panic!("unknown axis"),
        }, parts[1].parse::<i32>().unwrap()));
    }
    
    let result = fold_all(points, folds);
    let x_max = result.iter().map(|p| p.0).max().unwrap();
    let y_max = result.iter().map(|p| p.1).max().unwrap();
    for y in 0..=y_max {
        for x in 0..=x_max {
            print!("{}", if result.contains(&(x, y)) { '#' } else { ' ' });
        }
        println!();
    }
}
