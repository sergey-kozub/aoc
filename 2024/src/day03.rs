use regex::Regex;

fn mul_add(text: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(text).map(|c| {
        c.iter().skip(1).map(|m| m.unwrap().as_str())
            .map(|s| s.parse::<u32>().unwrap())
            .product::<u32>()
    }).sum::<u32>()
}

fn mul_skip_add(text: &str) -> u32 {
    text.split("do()").map(|part| {
        let t = part.split_once("don't()");
        mul_add(t.map(|x| x.0).unwrap_or(part))
    }).sum::<u32>()
}

pub fn run(content: &str) {
    println!("{} {}", mul_add(content), mul_skip_add(content));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)\
                    +mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(super::mul_add(test), 161);
    }

    #[test]
    fn large() {
        let test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)\
                    +mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(super::mul_skip_add(test), 48);
    }
}
