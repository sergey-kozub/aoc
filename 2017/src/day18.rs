use std::collections::VecDeque;

#[derive(Debug)]
pub enum Operand {
    Reg(u8),
    Imm(i32),
}

#[derive(Debug)]
pub enum Instruction {
    Snd(u8),
    Set(u8, Operand),
    Add(u8, Operand),
    Sub(u8, Operand),
    Mul(u8, Operand),
    Mod(u8, Operand),
    Rcv(u8),
    Jgz(Operand, Operand),
    Jnz(Operand, Operand),
}

#[derive(Debug)]
pub struct Program {
    pub code: Vec<Instruction>,
    pub ip: usize,
    pub reg: Vec<i64>,
    pub queue: VecDeque<i64>,
}

impl Instruction {
    fn parse(text: &str) -> Self {
        let first = |s: &str| s.chars().next().unwrap();
        let parse_reg = |s| first(s).to_digit(36).unwrap() as u8 - 10;
        let parse_op = |s| {
            if first(s).is_ascii_lowercase() {
                Operand::Reg(parse_reg(s))
            } else {
                Operand::Imm(s.parse::<i32>().unwrap())
            }
        };

        let a = text.split(" ").collect::<Vec<_>>();
        let reg = || parse_reg(a[1]);
        let op = || parse_op(a[2]);
        match a[0] {
            "snd" => Instruction::Snd(reg()),
            "set" => Instruction::Set(reg(), op()),
            "add" => Instruction::Add(reg(), op()),
            "sub" => Instruction::Sub(reg(), op()),
            "mul" => Instruction::Mul(reg(), op()),
            "mod" => Instruction::Mod(reg(), op()),
            "rcv" => Instruction::Rcv(reg()),
            "jgz" => Instruction::Jgz(parse_op(a[1]), op()),
            "jnz" => Instruction::Jnz(parse_op(a[1]), op()),
            _ => panic!(),
        }
    }
}

impl Program {
    pub fn parse(text: &str, regs: usize) -> Self {
        let code = text.lines().map(|line| Instruction::parse(line)).collect();
        Self { code, ip: 0, reg: vec![0; regs], queue: VecDeque::new() }
    }

    fn simple(&mut self) -> Option<i64> {
        loop {
            if let Some(n) = self.next()? {
                if self.reg[n as usize] != 0 {
                    return self.queue.pop_back();
                }
            }
        }
    }

    fn run(&mut self) -> Option<usize> {
        while let Some(r) = self.next() {
            if r.is_some() { return r.map(|x| x as usize); }
        }
        None
    }
}

impl Iterator for Program {
    type Item = Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ip >= self.code.len() { return None; }
        let get = |op: &Operand| match op {
            Operand::Reg(i) => self.reg[*i as usize],
            Operand::Imm(v) => *v as i64,
        };

        let mut offset = 1;
        match &self.code[self.ip] {
            Instruction::Snd(n) => self.queue.push_back(self.reg[*n as usize]),
            Instruction::Set(n, op) => self.reg[*n as usize] = get(op),
            Instruction::Add(n, op) => self.reg[*n as usize] += get(op),
            Instruction::Sub(n, op) => self.reg[*n as usize] -= get(op),
            Instruction::Mul(n, op) => self.reg[*n as usize] *= get(op),
            Instruction::Mod(n, op) => self.reg[*n as usize] %= get(op),
            Instruction::Rcv(n) => { self.ip += 1; return Some(Some(*n)); },
            Instruction::Jgz(l, r) if get(l) > 0 => offset = get(r),
            Instruction::Jnz(l, r) if get(l) != 0 => offset = get(r),
            _ => {},
        };

        let next = self.ip as isize + offset as isize;
        let ok = next >= 0 && next < self.code.len() as isize;
        self.ip = if ok {next as usize} else {usize::MAX};
        if ok {Some(None)} else {None}
    }
}

fn twin_run(text: &str) -> usize {
    let mut count = 0;
    let mut p0 = Program::parse(text, 16);
    let mut p1 = Program::parse(text, 16);
    p1.reg[15] = 1;

    let (mut r0, mut r1) = (p0.run(), p1.run());
    loop {
        let mut updated = false;
        while r0.is_some() && !p1.queue.is_empty() {
            p0.reg[r0.unwrap()] = p1.queue.pop_front().unwrap();
            r0 = p0.run();
            updated = true;
            count += 1;
        }
        while r1.is_some() && !p0.queue.is_empty() {
            p1.reg[r1.unwrap()] = p0.queue.pop_front().unwrap();
            r1 = p1.run();
            updated = true;
        }
        if !updated { break; }
    }
    count + p1.queue.len()
}

pub fn run(content: &str) {
    let mut duet = Program::parse(content, 16);
    println!("{} {}", duet.simple().unwrap(), twin_run(content));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        set a 1\n\
        add a 2\n\
        mul a a\n\
        mod a 5\n\
        snd a\n\
        set a 0\n\
        rcv a\n\
        jgz a -1\n\
        set a 1\n\
        jgz a -2";

    #[test]
    fn small() {
        let mut test = super::Program::parse(TEST, 1);
        assert_eq!(test.simple(), Some(4));
    }
}
