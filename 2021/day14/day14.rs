use std::collections::HashMap;
use std::fs;

fn process(template: &str, rules: &Vec<(String, char)>, steps: usize) -> String {
    let mut result = String::from(template);
    for _ in 0..steps {
        let clist: Vec<char> = result.chars().collect();
        let mut chunks: Vec<String> = result.chars().map(String::from).collect();
        for (sub, ins) in rules {
            let m: Vec<char> = sub.chars().collect();
            for i in 1..result.len() {
                if clist[i - 1] == m[0] && clist[i] == m[1] {
                    chunks[i - 1].push(*ins);
                }
            }
        }
        result = chunks.into_iter().collect();
    }
    result
}

fn score(polymer: String) -> usize {
    let mut count: HashMap<char, usize> = HashMap::new();
    for ch in polymer.chars() {
        *count.entry(ch).or_insert(0) += 1;
    }
    let count_min = count.values().min().unwrap();
    let count_max = count.values().max().unwrap();
    return count_max - count_min;
}

fn process_score(template: &str, rules: &Vec<(String, char)>, steps: usize) -> usize {
    let concat = |a: char, b: char| -> String { String::from(a) + &String::from(b) };
    let mut count: HashMap<String, isize> = HashMap::new();

    let first = template.chars().next().unwrap();
    let mut last: char = first;
    for ch in template.chars().skip(1) {
        *count.entry(concat(last, ch)).or_insert(0) += 1;
        last = ch;
    }
    for _ in 0..steps {
        let mut mods: Vec<(String, isize)> = Vec::new();
        for (sub, ins) in rules {
            if let Some(c) = count.get(sub) {
                let m: Vec<char> = sub.chars().collect();
                mods.push((concat(m[0], *ins), *c));
                mods.push((concat(*ins, m[1]), *c));
                mods.push((sub.to_string(), -*c));
            }
        }
        for (sub, val) in mods {
            *count.entry(sub).or_insert(0) += val;
        }
    }

    let mut result: HashMap<char, usize> = HashMap::new();
    for (sub, val) in count {
        for ch in sub.chars() {
            *result.entry(ch).or_insert(0) += val as usize;
        }
    }
    *result.entry(first).or_insert(0) += 1;
    *result.entry(last).or_insert(0) += 1;
    let count_min = result.values().min().unwrap();
    let count_max = result.values().max().unwrap();
    return (count_max - count_min) / 2;
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").expect("Error reading input")
        .split("\n\n").map(String::from).collect();

    let template: String = input[0].clone();
    let rules: Vec<(String, char)> = input[1].lines().map(|s| {
        let parts: Vec<&str> = s.split(" -> ").collect();
        (String::from(parts[0]), parts[1].chars().next().unwrap())
    }).collect();

    let res_1 = score(process(&template, &rules, 10));
    let res_2 = process_score(&template, &rules, 40);
    println!("{} {}", res_1, res_2)
}
