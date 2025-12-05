
fn parse(line: &str) -> Vec<u8> {
    line.chars().map(|c| c.to_digit(2).unwrap() as u8).collect()
}

fn fill(mut data: Vec<u8>, limit: usize) -> Vec<u8> {
    while data.len() <= limit {
        let tail = data.iter().rev().map(|&v| 1 - v).collect::<Vec<_>>();
        data = data.into_iter().chain([0].into_iter()).chain(tail.into_iter())
            .collect::<Vec<_>>();
    }
    data.truncate(limit);
    data
}

fn checksum(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 2 == 0 {
        data = data.chunks(2).map(|a| if a[0] == a[1] {1} else {0})
            .collect::<Vec<_>>();
    }
    data
}

fn format(data: Vec<u8>) -> String {
    data.into_iter().map(|x| if x != 0 {'1'} else {'0'}).collect()
}

pub fn run(content: &str) {
    let res1 = format(checksum(fill(parse(content), 272)));
    let res2 = format(checksum(fill(parse(content), 35651584)));
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    static P: fn(&str) -> Vec<u8> = super::parse;

    #[test]
    fn small() {
        assert_eq!(super::fill(P("10000"), 20), P("10000011110010000111"));
        assert_eq!(super::checksum(P("10000011110010000111")), P("01100"));
    }
}
