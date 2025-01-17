use std::collections::HashMap;

#[derive(Debug)]
enum Compare {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Ne,
}

#[derive(Debug)]
struct Instruction {
    mod_var: String,
    mod_value: i32,
    cond_var: String,
    cond_op: Compare,
    cond_value: i32,
}

struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<String, i32>,
    highest: i32,
}

impl Instruction {
    fn parse(text: &str) -> Self {
        let a = text.split(' ').collect::<Vec<_>>();
        assert!(a.len() == 7 && a[3] == "if");
        let op = match a[5] {
            "<" => Compare::Lt,
            "<=" => Compare::Lte,
            ">" => Compare::Gt,
            ">=" => Compare::Gte,
            "==" => Compare::Eq,
            "!=" => Compare::Ne,
            _ => panic!("unknown op"),
        };
        let sign = match a[1] {
            "inc" => 1,
            "dec" => -1,
            _ => panic!("unknown mod"),
        };
        Self {
            mod_var: a[0].to_owned(),
            mod_value: sign * a[2].parse::<i32>().unwrap(),
            cond_var: a[4].to_owned(),
            cond_op: op,
            cond_value: a[6].parse::<i32>().unwrap(),
        }
    }

    fn matches(&self, value: i32) -> bool {
        match self.cond_op {
            Compare::Lt => value < self.cond_value,
            Compare::Lte => value <= self.cond_value,
            Compare::Gt => value > self.cond_value,
            Compare::Gte => value >= self.cond_value,
            Compare::Eq => value == self.cond_value,
            Compare::Ne => value != self.cond_value,
        }
    }
}

impl Program {
    fn parse(text: &str) -> Self {
        let instructions = text.lines()
            .map(Instruction::parse).collect::<Vec<_>>();
        Self { instructions, registers: HashMap::new(), highest: 0 }
    }

    fn run(&mut self) -> i32 {
        for item in &self.instructions {
            let value = *self.registers.get(&item.cond_var).unwrap_or(&0);
            if item.matches(value) {
                let value = self.registers.entry(item.mod_var.clone())
                    .and_modify(|x| *x += item.mod_value)
                    .or_insert(item.mod_value);
                self.highest = self.highest.max(*value);
            }
        }
        *self.registers.values().max().unwrap()
    }
}

pub fn run(content: &str) {
    let mut program = Program::parse(content);
    println!("{} {}", program.run(), program.highest);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        b inc 5 if a > 1\n\
        a inc 1 if b < 5\n\
        c dec -10 if a >= 1\n\
        c inc -20 if c == 10";

    #[test]
    fn small() {
        let mut program = super::Program::parse(TEST);
        assert_eq!(program.run(), 1);
    }

    #[test]
    fn large() {
        let mut program = super::Program::parse(TEST);
        program.run();
        assert_eq!(program.highest, 10);
    }
}
