use crate::intcode::IntCode;

fn modify_run(program: &str, noun: i64, verb: i64) -> i64 {
    let mut test = IntCode::from(program);
    test.set(1, noun);
    test.set(2, verb);
    test.run().unwrap()[0]
}

fn find_pair(program: &str, expect: i64) -> Option<i64> {
    for noun in 0..100 {
        for verb in 0..100 {
            if modify_run(program, noun, verb) == expect {
                return Some(100 * noun + verb)
            }
        }
    }
    None
}

pub fn run(content: &str) {
    let res_1 = modify_run(content, 12, 2);
    let res_2 = find_pair(content, 19690720).unwrap();
    println!("{} {}", res_1, res_2)
}

#[cfg(test)]
mod tests {
    fn run(program: &str) -> Vec<i64> {
        let mut test = super::IntCode::from(program);
        test.run().unwrap().iter().map(|&x| x).collect()
    }

    #[test]
    fn intcode() {
        assert_eq!(
            run("1,9,10,3,2,3,11,0,99,30,40,50"),
            vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(
            run("1,0,0,0,99"),
            vec![2,0,0,0,99]);
        assert_eq!(
            run("2,3,0,3,99"),
            vec![2,3,0,6,99]);
        assert_eq!(
            run("2,4,4,5,99,0"),
            vec![2,4,4,5,99,9801]);
        assert_eq!(
            run("1,1,1,4,99,5,6,0,99"),
            vec![30,1,1,4,2,5,6,0,99]);
    }
}
