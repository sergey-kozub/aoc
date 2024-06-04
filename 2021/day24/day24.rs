use std::collections::HashMap;
use std::fs;
use std::ops::Range;

#[derive(Debug)]
enum Register { W, X, Y, Z }

#[derive(Debug)]
enum Value {
    Reg(Register),
    Imm(i32),
}

#[derive(Debug)]
enum Instruction {
    Inp(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Eql(Register, Value),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl State {
    fn get(&self, reg: &Register) -> i32 {
        match reg {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn set(&mut self, reg: &Register, val: i32) {
        match reg {
            Register::W => self.w = val,
            Register::X => self.x = val,
            Register::Y => self.y = val,
            Register::Z => self.z = val,
        }
    }

    fn update(&mut self, target: &Register, value: &Value, f: fn(i32, i32) -> i32) {
        let a = self.get(target);
        let b = match value {
            Value::Reg(r) => self.get(r),
            Value::Imm(v) => *v,
        };
        self.set(target, f(a, b))
    }

    fn apply(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Add(r, v) => self.update(&r, &v, |a, b| a + b),
            Instruction::Mul(r, v) => self.update(&r, &v, |a, b| a * b),
            Instruction::Div(r, v) => self.update(&r, &v, |a, b| a / b),
            Instruction::Mod(r, v) => self.update(&r, &v, |a, b| a % b),
            Instruction::Eql(r, v) => self.update(&r, &v, |a, b| (a == b) as i32),
            _ => panic!("incorrect instruction"),
        }
    }
}

#[derive(Debug)]
struct Program(Vec<Instruction>);

impl Program {
    fn from(text: &str) -> Program {
        let parse = |s| {
            match s {
                "w" => Value::Reg(Register::W),
                "x" => Value::Reg(Register::X),
                "y" => Value::Reg(Register::Y),
                "z" => Value::Reg(Register::Z),
                _ => Value::Imm(s.parse::<i32>().unwrap()),
            }
        };
        Program(text.lines().map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let target = match parse(parts[1]) {
                Value::Reg(register) => register,
                _ => panic!("incorrect target"),
            };
            match parts[0] {
                "inp" => Instruction::Inp(target),
                "add" => Instruction::Add(target, parse(parts[2])),
                "mul" => Instruction::Mul(target, parse(parts[2])),
                "div" => Instruction::Div(target, parse(parts[2])),
                "mod" => Instruction::Mod(target, parse(parts[2])),
                "eql" => Instruction::Eql(target, parse(parts[2])),
                _ => panic!("unknown instruction"),
            }
        }).collect())
    }

    fn get_ranges(&self) -> Vec<Range<usize>> {
        let mut inputs: Vec<usize> = self.0.iter().enumerate().filter_map(
            |(k, v)| if let Instruction::Inp(_) = v {Some(k)} else {None}).collect();
        inputs.push(self.0.len());
        inputs.windows(2).map(|a| a[0]..a[1]).collect()
    }

    fn run_range(&self, range: Range<usize>, mut state: State, input: i32) -> State {
        for inst in &self.0[range] {
            match inst {
                Instruction::Inp(r) => state.set(&r, input),
                _ => state.apply(inst),
            };
        }
        state
    }
}

fn search(program: &Program, sign: i64) -> i64 {
    let mut states: HashMap<State, i64> = HashMap::new();
    states.insert(State { w: 0, x: 0, y: 0, z: 0 }, 0);

    for range in program.get_ranges() {
        let mut next: HashMap<State, i64> = HashMap::new();
        for digit in 1..=9 {
            for (state, result) in states.iter() {
                let key = program.run_range(range.clone(), (*state).clone(), digit);
                let value = result * 10 + sign * digit as i64;
                if let Some(&existing) = next.get(&key) {
                    if existing > value { continue; }
                }
                next.insert(key, value);
            }
        }
        states = next;
        println!("{:?} {}", range, states.len());
    }
    states.iter().filter_map(|(k, v)| if k.z == 0 {Some(*v)} else {None})
        .max().unwrap() * sign
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input");
    let program = Program::from(&input);
    println!("{} {}", search(&program, 1), search(&program, -1));
}
