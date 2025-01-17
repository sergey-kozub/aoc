use crate::day18::{Instruction, Program};

fn count_matching<F>(code: &str, pred: F) -> usize
where F: Fn(&Instruction) -> bool {
    let mut result = 0;
    let mut inst = Program::parse(code, 8);
    loop {
        if pred(&inst.code[inst.ip]) { result += 1; }
        if inst.next().is_none() { return result; }
    }
}

fn is_prime(value: u32) -> bool {
    if value % 2 == 0 { return false; }
    (3..=value / 2).step_by(2).all(|x| value % x != 0)
}

pub fn run(content: &str) {
    let (start, end, step) = (109900, 126900, 17);
    let v1 = count_matching(content, |c| matches!(c, Instruction::Mul(_, _)));
    let v2 = (start..=end).step_by(step).filter(|&x| !is_prime(x)).count();
    println!("{} {}", v1, v2);
}
