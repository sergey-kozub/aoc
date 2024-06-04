use std::fs;

type Snail = Vec<(u32, u8)>;

fn parse(text: &str) -> Option<Snail> {
    let mut result: Snail = Vec::new();
    let mut level: u8 = 0;
    for ch in text.as_bytes() {
        match ch {
            b'0'..=b'9' => result.push(((ch - b'0') as u32, level)),
            b'[' => level += 1,
            b']' => level -= 1,
            b',' => {},
            _ => return None,
        }
    }
    if level == 0 { Some(result) } else { None }
}

fn explode(a: &mut Snail, n: u8) -> bool {
    for i in 1..a.len() {
        if a[i].1 > n && a[i].1 == a[i - 1].1 {
            if i >= 2 { a[i - 2].0 += a[i - 1].0; }
            if i < a.len() - 1 { a[i + 1].0 += a[i].0; }
            a[i] = (0, a[i].1 - 1);
            a.remove(i - 1);
            return true;
        }
    }
    false
}

fn split(a: &mut Snail, n: u32) -> bool {
    for i in 0..a.len() {
        if a[i].0 > n {
            a.insert(i, (a[i].0 / 2, a[i].1 + 1));
            a[i + 1] = (a[i + 1].0 - a[i].0, a[i].1);
            return true;
        }
    }
    false
}

fn magnitude(mut a: Snail) -> u32 {
    while a.len() > 1 {
        for i in 1..a.len() {
            if a[i].1 == a[i - 1].1 {
                a[i] = (a[i - 1].0 * 3 + a[i].0 * 2, a[i].1 - 1);
                a.remove(i - 1);
                break;
            }
        }
    }
    a[0].0
}

fn add(mut a: Snail, mut b: Snail) -> Snail {
    a.append(&mut b);
    for i in 0..a.len() { a[i].1 += 1; }
    loop {
        if !explode(&mut a, 4) && !split(&mut a, 9) { break; }
    }
    a
}

fn main() {
    let input: Vec<Snail> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(|s| parse(s).unwrap()).collect();

    let sum = magnitude(input.clone().into_iter().reduce(add).unwrap());
    let largest = input.iter().enumerate().flat_map(|(i, a)|
        input.iter().enumerate().map(move |(j, b)| {
            if i != j { magnitude(add(a.clone(), b.clone())) } else { 0 }
        })
    ).max().unwrap();
    println!("{} {}", sum, largest)
}
