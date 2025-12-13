
#[derive(Clone, Copy, Debug)]
pub enum Value {
    Imm(i64),
    Reg(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
    Out(Value),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub code: Vec<Instruction>,
    pub regs: Vec<i64>,
    pub ip: i64,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let as_value = |s: &str| {
            let first = s.chars().next().unwrap();
            if first.is_ascii_lowercase() {
                Value::Reg(first as u8 - 'a' as u8)
            } else {
                Value::Imm(s.parse::<i64>().unwrap())
            }
        };

        let a = line.split(' ').collect::<Vec<_>>();
        match a[0] {
            "cpy" => Self::Cpy(as_value(a[1]), as_value(a[2])),
            "inc" => Self::Inc(as_value(a[1])),
            "dec" => Self::Dec(as_value(a[1])),
            "jnz" => Self::Jnz(as_value(a[1]), as_value(a[2])),
            "tgl" => Self::Tgl(as_value(a[1])),
            "out" => Self::Out(as_value(a[1])),
            _ => panic!(),
        }
    }
}

impl Program {
    pub fn new(text: &str) -> Self {
        Self {
            code: text.lines().map(Instruction::parse).collect(),
            regs: vec![0; 4],
            ip: 0,
        }
    }

    pub fn get(&self, value: &Value) -> i64 {
        match value {
            Value::Imm(v) => *v,
            Value::Reg(i) => self.regs[*i as usize],
        }
    }

    pub fn step(&mut self) -> bool {
        let limit = self.code.len() as i64;
        let valid = |ip| ip >= 0 && ip < limit;
        match self.code[self.ip as usize] {
            Instruction::Cpy(src, dst) => {
                if let Value::Reg(i) = dst {
                    self.regs[i as usize] = self.get(&src);
                }
            },
            Instruction::Inc(dst) => {
                if let Value::Reg(i) = dst {
                    self.regs[i as usize] += 1;
                }
            },
            Instruction::Dec(dst) => {
                if let Value::Reg(i) = dst {
                    self.regs[i as usize] -= 1;
                }
            },
            Instruction::Jnz(chk, val) => {
                if self.get(&chk) != 0 {
                    self.ip += self.get(&val) - 1;
                }
            },
            Instruction::Tgl(val) => {
                let ptr = self.ip + self.get(&val);
                if valid(ptr) {
                    let instr = &mut self.code[ptr as usize];
                    *instr = match *instr {
                        Instruction::Cpy(x, y) => Instruction::Jnz(x, y),
                        Instruction::Inc(x) => Instruction::Dec(x),
                        Instruction::Dec(x) => Instruction::Inc(x),
                        Instruction::Jnz(x, y) => Instruction::Cpy(x, y),
                        Instruction::Tgl(x) => Instruction::Inc(x),
                        Instruction::Out(x) => Instruction::Out(x),
                    };
                }
            },
            Instruction::Out(_) => {},
        }
        self.ip += 1;
        valid(self.ip)
    }

    fn run(mut self, init: i64) -> i64 {
        self.regs[0] = init;
        while self.step() {}
        self.regs[0]
    }
}

pub fn run(content: &str) {
    let program = Program::new(content);
    let res1 = program.clone().run(7);
    let res2 = program.clone().run(12);
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a";

    #[test]
    fn small() {
        assert_eq!(super::Program::new(TEST).run(0), 3);
    }
}
