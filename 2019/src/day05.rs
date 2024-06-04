use crate::intcode::IntCode;

pub fn run(content: &str) {
    let process = |input| IntCode::from(content).run_single(&[input]);
    println!("{} {}", process(1), process(5))
}

#[cfg(test)]
mod tests {
    fn run(program: &str, input: &[i64]) -> Vec<i64> {
        let mut test = super::IntCode::from(program);
        for v in input { test.input.push_back(*v); }
        test.run();
        test.output
    }

    #[test]
    fn intcode() {
        assert_eq!(run("3,0,4,0,99", &[1]), vec![1]);
        assert_eq!(run("3,0,4,0,99", &[-1]), vec![-1]);
        assert_eq!(run("1002,4,3,4,33", &[]), vec![]);

        for eq_8 in &["3,9,8,9,10,9,4,9,99,-1,8", "3,3,1108,-1,8,3,4,3,99"] {
            assert_eq!(run(eq_8, &[8]), vec![1]);
            assert_eq!(run(eq_8, &[9]), vec![0]);
        }
        for le_8 in &["3,9,7,9,10,9,4,9,99,-1,8", "3,3,1107,-1,8,3,4,3,99"] {
            assert_eq!(run(le_8, &[7]), vec![1]);
            assert_eq!(run(le_8, &[8]), vec![0]);
        }
        for non_zero in &["3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9",
                          "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"] {
            assert_eq!(run(non_zero, &[0]), vec![0]);
            assert_eq!(run(non_zero, &[10]), vec![1]);
        }

        let comp_8 = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
                      1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
                      999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run(comp_8, &[0]), vec![999]);
        assert_eq!(run(comp_8, &[8]), vec![1000]);
        assert_eq!(run(comp_8, &[10]), vec![1001]);
    }
}
