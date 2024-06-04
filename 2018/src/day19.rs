use crate::day16::{Computer, OpCode, Value};

#[derive(Debug)]
pub struct Program {
  pub ip_index: usize,
  pub instructions: Vec<(OpCode, Value, Value, Value)>,
}

impl Program {
  fn get_opcode(name: &str) -> OpCode {
    match name {
      "addr" => OpCode::AddR,
      "addi" => OpCode::AddI,
      "mulr" => OpCode::MulR,
      "muli" => OpCode::MulI,
      "banr" => OpCode::BanR,
      "bani" => OpCode::BanI,
      "borr" => OpCode::BorR,
      "bori" => OpCode::BorI,
      "setr" => OpCode::SetR,
      "seti" => OpCode::SetI,
      "gtir" => OpCode::GtIR,
      "gtri" => OpCode::GtRI,
      "gtrr" => OpCode::GtRR,
      "eqir" => OpCode::EqIR,
      "eqri" => OpCode::EqRI,
      "eqrr" => OpCode::EqRR,
      _ => panic!("unknown opcode"),
    }
  }

  pub fn parse(text: &str) -> Self {
    let mut ip_index = 0_usize;
    let mut instructions = vec![];
    for line in text.lines() {
      if line.starts_with("#ip") {
        let (_, s) = line.split_once(' ').unwrap();
        ip_index = s.parse::<usize>().unwrap();
      } else {
        let a = line.split(' ').collect::<Vec<_>>();
        let v = a.iter().skip(1)
          .map(|s| s.parse::<Value>().unwrap())
          .collect::<Vec<_>>();
        let opcode = Self::get_opcode(a[0]);
        instructions.push((opcode, v[0], v[1], v[2]));
      }
    }
    Program { ip_index, instructions }
  }

  fn execute(&self, init: Value) -> Vec<Value> {
    let mut comp = Computer::new(6);
    let mut ip: Value = 0;
    comp.reg[0] = init;
    while (ip as usize) < self.instructions.len() {
      let (op, a, b, c) = self.instructions[ip as usize];
      comp.reg[self.ip_index] = ip;
      comp.execute(op, a, b, c);
      ip = comp.reg[self.ip_index] + 1;
    }
    comp.reg
  }
}

pub fn run(content: &str) {
  let program = Program::parse(content);
  let res1 = program.execute(0)[0];
  let sum_divisors = |n: usize| (1..=n).filter(|i| n/i*i==n).sum::<usize>();
  let res2 = sum_divisors(943 + 10550400);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

  #[test]
  fn small() {
    let program = super::Program::parse(TEST);
    assert_eq!(program.execute(0), [6, 5, 6, 0, 0, 9]);
  }
}
