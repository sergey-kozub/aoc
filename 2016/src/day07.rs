use std::collections::HashSet;

type Addr = Vec<String>;

fn parse(text: &str) -> Addr {
    text.split(|c| matches!(c, '[' | ']')).map(|s| s.into())
        .collect::<Vec<_>>()
}

fn is_tls(addr: &Addr) -> bool {
    let has_abba = |s: &str| -> bool {
        let a = s.as_bytes();
        (0..a.len() - 3).any(
            |i| a[i + 3] == a[i] && a[i + 2] == a[i + 1] && a[i + 1] != a[i])
    };
    let c1 = addr.iter().step_by(2).any(|v| has_abba(v));
    let c2 = addr.iter().skip(1).step_by(2).all(|v| !has_abba(v));
    c1 && c2
}

fn is_ssl(addr: &Addr) -> bool {
    let has_aba = |s: &str, a: u8, b: u8| {
        let x = s.as_bytes();
        (0..x.len() - 2).any(|i| x[i] == b && x[i + 1] == a && x[i + 2] == b)
    };
    addr.iter().step_by(2).any(|v| {
        let x = v.as_bytes();
        (0..x.len() - 2).any(|i| {
            let (a, b) = (x[i], x[i + 1]);
            x[i + 2] == a && a != b && addr.iter().skip(1).step_by(2)
                .any(|y| has_aba(y, a, b))
        })
    })
}

pub fn run(content: &str) {
    let data = content.lines().map(parse).collect::<Vec<_>>();
    let uniq = data.iter().cloned().collect::<HashSet<_>>();
    let count_1 = uniq.iter().filter(|&v| is_tls(v)).count();
    let count_2 = uniq.iter().filter(|&v| is_ssl(v)).count();
    println!("{} {}", count_1, count_2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let check = |s| super::is_tls(&super::parse(s));
        assert!(check("abba[mnop]qrst"));
        assert!(!check("abcd[bddb]xyyx"));
        assert!(!check("aaaa[qwer]tyui"));
        assert!(check("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn large() {
        let check = |s| super::is_ssl(&super::parse(s));
        assert!(check("aba[bab]xyz"));
        assert!(!check("xyx[xyx]xyx"));
        assert!(check("aaa[kek]eke"));
        assert!(check("zazbz[bzb]cdb"));
    }
}
