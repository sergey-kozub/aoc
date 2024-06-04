use crate::day16::{Computer, OpCode, Value};
use crate::day19::Program;
use std::collections::HashSet;

fn execute(program: &Program, last: bool) -> Value {
  let mut comp = Computer::new(6);
  let mut visited = HashSet::<Value>::new();
  let mut last_seen = 0;
  loop {
    let ip = comp.reg[program.ip_index] as usize;
    let (op, a, b, c) = program.instructions[ip];
    comp.execute(op, a, b, c);
    comp.reg[program.ip_index] += 1;
    if op == OpCode::EqRR && (a == 0 || b == 0) {
      let value = comp.reg[(a + b) as usize];
      if !last { return value; }
      if !visited.insert(value) { return last_seen; }
      last_seen = value;
    }
  }
}

pub fn run(content: &str) {
  let program = Program::parse(&content);
  let res1 = execute(&program, false);
  let res2 = execute(&program, true);
  println!("{} {}", res1, res2);
}
