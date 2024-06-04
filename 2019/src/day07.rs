use crate::intcode::IntCode;
use itertools::Itertools;

fn run_amplifiers(program: &str, phase: &[i64]) -> i64 {
    let count = phase.len();
    let mut amps: Vec<IntCode> = phase.iter().map(|init| {
        let mut amp = IntCode::from(program);
        amp.input.push_back(*init);
        amp
    }).collect();

    let mut steps: usize = 0;
    let mut signal: i64 = 0;
    loop {
        let amp = &mut amps[steps % count];
        amp.input.push_back(signal);
        match amp.wait() {
            Some(value) => signal = value,
            None => break,
        }
        steps += 1;
    }
    signal
}

fn max_output_simple(program: &str) -> i64 {
    (0..5).permutations(5).map(|perm| {
        perm.iter().fold(0, |a, b| {
            IntCode::from(program).run_single(&[*b, a])
        })
    }).max().unwrap()
}

fn max_output_feedback(program: &str) -> i64 {
    (5..10).permutations(5).map(|perm| {
        run_amplifiers(program, &perm)
    }).max().unwrap()
}

pub fn run(content: &str) {
    let signal_1 = max_output_simple(content);
    let signal_2 = max_output_feedback(content);
    println!("{} {}", signal_1, signal_2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(43210, super::max_output_simple(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"));
        assert_eq!(54321, super::max_output_simple(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
             101,5,23,23,1,24,23,23,4,23,99,0,0"));
        assert_eq!(65210, super::max_output_simple(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
             1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"));
    }

    #[test]
    fn part2() {
        assert_eq!(139629729, super::max_output_feedback(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
             27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"));
        assert_eq!(18216, super::max_output_feedback(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
             -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
             53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"));
    }
}
