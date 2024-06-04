use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct IntCode {
    program: Vec<i64>,
    memory: HashMap<usize, i64>,
    pub input: VecDeque<i64>,
    pub output: Vec<i64>,
    bp: isize,
    ip: usize,
}

enum RunState {
    Running,
    Block,
    Exit,
}

impl IntCode {
    pub fn create(program: Vec<i64>) -> IntCode {
        IntCode {
            program,
            memory: HashMap::new(),
            input: VecDeque::new(),
            output: Vec::new(),
            bp: 0,
            ip: 0,
        }
    }

    pub fn from(text: &str) -> IntCode {
        let program: Vec<i64> = text.trim_end().split(',').map(
            |x| x.parse::<i64>().unwrap()).collect();
        IntCode::create(program)
    }

    fn read(&self, addr: usize) -> i64 {
        if addr < self.program.len() {
            self.program[addr]
        } else {
            *self.memory.get(&addr).unwrap_or(&0)
        }
    }

    fn addr(&self, value: i64, mode: u16) -> usize {
        match mode {
            0 => value as usize,
            2 => (value as isize + self.bp) as usize,
            _ => panic!("unknown mode"),
        }
    }

    fn get(&self, value: i64, mode: u16) -> i64 {
        match mode {
            1 => value,
            _ => self.read(self.addr(value, mode)),
        }
    }

    fn step(&mut self) -> RunState {
        let mut write: Option<(usize, i64)> = None;
        {
            let packed = self.read(self.ip) as u16;
            let parts = [packed % 100,
                            packed / 100 % 10,
                            packed / 1000 % 10,
                            packed / 10000];
            let addr = |n| self.addr(self.read(self.ip + n), parts[n]);
            let get = |n| self.get(self.read(self.ip + n), parts[n]);

            match parts[0] {
                // sum
                1 => {
                    let value = get(1) + get(2);
                    write = Some((addr(3), value));
                    self.ip += 4;
                },
                // product
                2 => {
                    let value = get(1) * get(2);
                    write = Some((addr(3), value));
                    self.ip += 4;
                },
                // read input
                3 => {
                    let addr = addr(1);
                    match self.input.pop_front() {
                        Some(value) => write = Some((addr, value)),
                        None => return RunState::Block,
                    }
                    self.ip += 2;
                },
                // write output
                4 => {
                    let value = get(1);
                    self.output.push(value);
                    self.ip += 2;
                },
                // jump-if-true
                5 => {
                    if get(1) != 0 {
                        self.ip = get(2) as usize;
                        return RunState::Running;
                    }
                    self.ip += 3;
                },
                // jump-if-false
                6 => {
                    if get(1) == 0 {
                        self.ip = get(2) as usize;
                        return RunState::Running;
                    }
                    self.ip += 3;
                },
                // less than
                7 => {
                    let value = (get(1) < get(2)) as i64;
                    write = Some((addr(3), value));
                    self.ip += 4;
                },
                // equals
                8 => {
                    let value = (get(1) == get(2)) as i64;
                    write = Some((addr(3), value));
                    self.ip += 4;
                },
                // relative base offset
                9 => {
                    self.bp += get(1) as isize;
                    self.ip += 2;
                },
                // halt
                99 => return RunState::Exit,
                _ => panic!("unknown command"),
            }
        }

        if let Some((pos, val)) = write {
            if pos < self.program.len() {
                self.program[pos] = val;
            } else {
                self.memory.insert(pos, val);
            }
        }
        RunState::Running
    }

    pub fn wait(&mut self) -> Option<i64> {
        while let RunState::Running = self.step() {
            if !self.output.is_empty() { break; }
        }
        self.output.pop()
    }

    pub fn wait_many(&mut self, count: usize) -> Option<Vec<i64>> {
        let mut result = Vec::<i64>::with_capacity(count);
        for _ in 0..count { result.push(self.wait()?); }
        Some(result)
    }

    pub fn run(&mut self) -> Option<&[i64]> {
        loop {
            match self.step() {
                RunState::Running => continue,
                RunState::Block => return None,
                RunState::Exit => return Some(&self.program),
            }
        }
    }

    pub fn run_single(&mut self, input: &[i64]) -> i64 {
        for v in input { self.input.push_back(*v); }
        assert!(self.run().is_some());
        self.output.drain(..).last().unwrap()
    }

    pub fn set(&mut self, addr: usize, value: i64) {
        self.program[addr] = value;
    }

    pub fn is_active(&self) -> bool {
        self.program[self.ip] != 99
    }
}
