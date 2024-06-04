use crate::intcode::IntCode;

pub fn run(content: &str) {
    let process = |input| IntCode::from(content).run_single(&[input]);
    println!("{} {}", process(1), process(2))
}

#[cfg(test)]
mod tests {
    fn run(program: &str) -> Vec<i64> {
        let mut test = super::IntCode::from(program);
        test.run();
        test.output
    }

    #[test]
    fn intcode() {
        assert_eq!(
            run("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
            vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        assert_eq!(
            run("1102,34915192,34915192,7,4,7,99,0"),
            vec![1219070632396864]);
        assert_eq!(
            run("104,1125899906842624,99"),
            vec![1125899906842624]);
    }
}
