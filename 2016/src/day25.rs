use crate::day23::{Instruction, Program};
use std::collections::HashSet;

fn simulate(mut program: Program, init: i64) -> bool {
    let mut visited = HashSet::new();
    let mut emitted = vec![];
    program.regs[0] = init;
    while program.step() {
        if let Instruction::Out(val) = program.code[program.ip as usize] {
            emitted.push(program.get(&val));
        }
        if !visited.insert((program.ip, program.regs.clone())) {
            println!("{init}: loop {emitted:?}");
            return emitted.len() % 2 == 0 &&
                   emitted.chunks(2).all(|a| a[0] == 0 && a[1] == 1);
        }
    }
    false
}

pub fn run(content: &str) {
    let program = Program::new(content);
    for init in 1.. {
        if simulate(program.clone(), init) { break; }
    }
}
