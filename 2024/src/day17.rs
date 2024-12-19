
#[derive(Debug)]
enum Opcode { Adv, Bxl, Bst, Jnz, Bxc, Out, Bdv, Cdv }

#[derive(Debug)]
struct Computer {
    reg: [i64; 3],
    ip: usize,
    data: Vec<u8>,
}

impl Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!(),
        }
    }
}

impl Computer {
    fn new(reg: i64, data: Vec<u8>) -> Self {
        Self { reg: [reg, 0, 0], ip: 0, data }
    }

    fn parse(text: &str) -> Self {
        let (l, r) = text.split_once("\n\n").unwrap();
        let a = l.split("\n").map(|s| {
            s.split_once(": ").unwrap().1.parse::<i64>().unwrap()
        }).collect::<Vec<_>>();
        let data = r.split_once(": ").unwrap().1.split(",").map(|v| {
            v.parse::<u8>().unwrap()
        }).collect::<Vec<_>>();
        Self { reg: [a[0], a[1], a[2]], ip: 0, data }
    }

    fn combo(&self, value: u8) -> i64 {
        match value {
            0..=3 => value as i64,
            4..=6 => self.reg[value as usize - 4],
            _ => panic!(),
        }
    }

    fn process(&mut self, op: Opcode, value: u8) -> Option<u8> {
        self.ip += 2;
        let denom = || 2_i64.pow(self.combo(value) as u32);
        match op {
            Opcode::Adv => self.reg[0] /= denom(),
            Opcode::Bxl => self.reg[1] ^= value as i64,
            Opcode::Bst => self.reg[1] = self.combo(value) % 8,
            Opcode::Jnz => if self.reg[0] != 0 { self.ip = value as usize },
            Opcode::Bxc => self.reg[1] ^= self.reg[2],
            Opcode::Out => return Some((self.combo(value) % 8) as u8),
            Opcode::Bdv => self.reg[1] = self.reg[0] / denom(),
            Opcode::Cdv => self.reg[2] = self.reg[0] / denom(),
        }
        None
    }

    fn search(&self, mut start: usize) -> usize {
        loop {
            let mut inst = Computer::new(start as i64, self.data.clone());
            if self.data.iter().all(|&v| inst.next() == Some(v)) &&
                inst.next().is_none() { return start; }
            start += 1;
        }
    }
}

impl Iterator for Computer {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        loop {
            if self.ip >= self.data.len() { return None; }
            let a = &self.data[self.ip..];
            let b = self.process(Opcode::from(a[0]), a[1]);
            if b.is_some() { return b; }
        }
    }
}

fn descend(data: &[u8], part: &[u8], tail: usize) -> Option<usize> {
    if data.is_empty() {
        return if tail == 0 {Some(0)} else {None};
    }
    part.iter().enumerate().skip(tail).step_by(128).filter_map(|(k, v)| {
        if *v != data[0] { return None; }
        descend(&data[1..], part, k / 8).map(|x| x * 8 + k % 8)
    }).min()
}

fn heuristic(data: Vec<u8>) -> Option<usize> {
    let part = (0..1024).map(|i| {
        Computer::new(i, data.clone()).next().unwrap()
    }).collect::<Vec<_>>();
    (0..128).filter_map(|i| descend(&data, &part, i)).min()
}

pub fn run(content: &str) {
    let inst = Computer::parse(content);
    let start = heuristic(inst.data.clone()).unwrap();
    let val = inst.search(start);
    let out = inst.map(|x| x.to_string()).collect::<Vec<_>>();
    println!("{} {}", out.join(","), val);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let inst = super::Computer::new(729, vec![0,1,5,4,3,0]);
        let out = inst.collect::<Vec<_>>();
        assert_eq!(out, vec![4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn large() {
        let inst = super::Computer::new(0, vec![0,3,5,4,3,0]);
        assert_eq!(inst.search(0), 117440);
    }
}
