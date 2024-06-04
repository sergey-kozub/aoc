use crate::intcode::IntCode;

fn check(program: &str, input: &str) -> Option<i64> {
    let mut cpu = IntCode::from(program);
    for ch in input.as_bytes().iter() {
        cpu.input.push_back(*ch as i64);
    }
    while let Some(value) = cpu.wait() {
        if value > u8::MAX as i64 {
            return Some(value);
        }
        print!("{}", value as u8 as char);
    }
    None
}

pub fn run(content: &str) {
    let simple = "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\n";
    let walk = check(content, &format!("{}WALK\n", simple));
    let run = check(content, &format!("{}AND H J\nNOT A T\nOR T J\nRUN\n", simple));
    println!("{} {}", walk.unwrap(), run.unwrap());
}
