
#[derive(Clone, Debug)]
enum Instruction {
    CopyImm(i64, u8),
    CopyReg(u8, u8),
    Inc(u8),
    Dec(u8),
    JumpCond(u8, i64),
    Jump(i64),
}

#[derive(Clone, Debug)]
struct Program {
    code: Vec<Instruction>,
    regs: Vec<i64>,
    ip: usize,
}

impl Instruction {
    fn parse(text: &str) -> Self {
        let to_reg = |s: &str| s.chars().next().unwrap() as u8 - 'a' as u8;
        let to_val = |s: &str| s.parse::<i64>().unwrap();
        let first = |s: &str| s.chars().next().unwrap();

        let a = text.split(' ').collect::<Vec<_>>();
        match a[0] {
            "cpy" => if first(a[1]).is_ascii_lowercase() {
                Self::CopyReg(to_reg(a[1]), to_reg(a[2]))
            } else {
                Self::CopyImm(to_val(a[1]), to_reg(a[2]))
            },
            "inc" => Self::Inc(to_reg(a[1])),
            "dec" => Self::Dec(to_reg(a[1])),
            "jnz" => if first(a[1]).is_ascii_lowercase() {
                Self::JumpCond(to_reg(a[1]), to_val(a[2]))
            } else {
                Self::Jump(to_val(a[2]))
            },
            _ => panic!(),
        }
    }
}

impl Program {
    fn new(text: &str) -> Self {
        Self {
            code: text.lines().map(Instruction::parse).collect(),
            regs: vec![0; 4],
            ip: 0,
        }
    }

    fn step(&mut self) {
        let jump = |v: i64| self.ip.overflowing_add(v as usize).0;
        match self.code[self.ip] {
            Instruction::CopyImm(v, r) => self.regs[r as usize] = v,
            Instruction::CopyReg(x, r) =>
                self.regs[r as usize] = self.regs[x as usize],
            Instruction::Inc(r) => self.regs[r as usize] += 1,
            Instruction::Dec(r) => self.regs[r as usize] -= 1,
            Instruction::JumpCond(r, v) =>
                if self.regs[r as usize] != 0 { self.ip = jump(v); return },
            Instruction::Jump(v) => { self.ip = jump(v); return },
        }
        self.ip += 1;
    }

    fn run(mut self) -> Vec<i64> {
        while self.ip < self.code.len() {
            self.step();
        }
        self.regs
    }
}

pub fn run(content: &str) {
    let mut program = Program::new(content);
    let res1 = program.clone().run()[0];
    program.regs[2] = 1;
    let res2 = program.run()[0];
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";

    #[test]
    fn small() {
        assert_eq!(super::Program::new(TEST).run()[0], 42);
    }
}
