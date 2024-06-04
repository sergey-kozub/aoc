
fn parse(s: &str) -> i64 {
    let mut base: i64 = 1;
    s.chars().rev().map(|c| {
        let res = base * match c {
            '0'|'1'|'2' => c.to_digit(10).unwrap() as i64,
            '-' => -1,
            '=' => -2,
            _ => panic!()
        };
        base *= 5;
        res
    }).sum()
}

fn format(mut n: i64) -> String {
    let (mut base, mut acc) = (1_i64, 0_i64);
    while n > acc {
        acc += base * 2;
        base *= 5;
    }
    let mut res = String::new();
    while base > 1 {
        base /= 5;
        acc -= base * 2;
        let mut digit = n / base;
        if (n - digit * base).abs() > acc {
            digit += n.signum();
        }
        n -= digit * base;
        res.push(match digit {
            -2 => '=',
            -1 => '-',
            0|1|2 => char::from_digit(digit as u32, 10).unwrap(),
            _ => panic!()
        });
    }
    res
}

pub fn run(content: &str) {
    let sum = content.lines().map(|s| parse(s.trim())).sum::<i64>();
    println!("{}", format(sum));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn snafu() {
        let nums: Vec<&str> = vec![
            "1=-0-2", "12111", "2=0=", "21", "2=01",
            "111", "20012", "112", "1=-1=", "1-12",
            "12", "1=", "122",
        ];
        let sum = nums.iter().cloned().map(super::parse).sum::<i64>();
        assert_eq!(sum, 4890);
        assert_eq!(super::format(sum), "2=-1=0");
    }
}
