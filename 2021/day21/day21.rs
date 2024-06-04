use std::cmp;
use std::collections::HashMap;

const PLAYER_1: u32 = 3;
const PLAYER_2: u32 = 5;

struct Die {
    value: u32,
    limit: u32,
}

impl Iterator for Die {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.value = self.value % self.limit + 1;
        Some(self.value)
    }
}

fn play_deterministic(limit: u32) -> (u32, u32, usize) {
    let mut die = Die { value: 0, limit: 100 };
    let mut pos = [PLAYER_1, PLAYER_2];
    let mut score = [0_u32; 2];
    let mut steps: usize = 0;

    loop {
        let roll: u32 = (&mut die).take(3).sum();
        let index = steps % 2;
        pos[index] = (pos[index] + roll - 1) % 10 + 1;
        score[index] += pos[index];
        steps += 3;
        if score[index] >= limit { break; }
    }
    (score[0], score[1], steps)
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    pos_1: u8,
    score_1: u8,
    pos_2: u8,
    score_2: u8,
}

fn play_quantum(limit: u8) -> (u64, u64) {
    let normal: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];
    let mut result = [0_u64; 2];
    let mut steps: usize = 0;
    let mut current: HashMap<State, u64> = HashMap::new();
    current.insert(State {
        pos_1: PLAYER_1 as u8,
        score_1: 0,
        pos_2: PLAYER_2 as u8,
        score_2: 0,
    }, 1);

    while !current.is_empty() {
        let first = steps % 2 == 0;
        let mut next: HashMap<State, u64> = HashMap::new();
        for (key, value) in current.iter() {
            let pos = if first {key.pos_1} else {key.pos_2};
            let score = if first {key.score_1} else {key.score_2};
            for sum in 3..=9 {
                let next_pos = (pos + sum - 1) % 10 + 1;
                let next_score = score + next_pos;
                let next_value = value * normal[sum as usize - 3];

                if next_score >= limit {
                    result[!first as usize] += next_value;
                    continue;
                }
                let next_key = if first {
                    State { pos_1: next_pos, score_1: next_score, ..*key }
                } else {
                    State { pos_2: next_pos, score_2: next_score, ..*key }
                };
                *next.entry(next_key).or_default() += next_value;
            }
        }
        steps += 1;
        current = next;
    }
    (result[0], result[1])
}

fn main() {
    let (s1, s2, n) = play_deterministic(1000);
    let (u1, u2) = play_quantum(21);
    println!("{} {}", cmp::min(s1, s2) * n as u32, cmp::max(u1, u2))
}
