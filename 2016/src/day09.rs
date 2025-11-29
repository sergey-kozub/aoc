
struct Compressed(String);
struct CompIter<'a>(&'a [u8], usize);

impl Compressed {
    fn iter(&self) -> CompIter<'_> {
        CompIter(&self.0.as_bytes(), 0)
    }
}

impl<'a> Iterator for CompIter<'a> {
    type Item = (&'a str, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == self.0.len() {
            return None;
        }
        let (s, n) = if self.0[self.1] == b'(' {
            let rest = &self.0[self.1..];
            let p1 = rest.iter().position(|&c| c == b'x').unwrap();
            let p2 = rest.iter().position(|&c| c == b')').unwrap();
            let n1 = str::from_utf8(&rest[1..p1]).unwrap()
                .parse::<usize>().unwrap();
            let n2 = str::from_utf8(&rest[p1 + 1..p2]).unwrap()
                .parse::<usize>().unwrap();
            self.1 += p2 + n1 + 1;
            (&rest[p2 + 1..p2 + n1 + 1], n2)
        } else {
            self.1 += 1;
            (&self.0[self.1 - 1..self.1], 1)
        };
        Some((str::from_utf8(s).unwrap(), n))
    }
}

fn count(text: &str) -> usize {
    Compressed(text.into()).iter().map(|(s, n)| s.len() * n).sum::<usize>()
}

fn rec(text: &str) -> usize {
    if text.len() == 1 { return 1; }
    Compressed(text.into()).iter().map(|(s, n)| rec(s) * n).sum::<usize>()
}

pub fn run(content: &str) {
    println!("{} {}", count(content), rec(content));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(super::count("ADVENT"), 6);
        assert_eq!(super::count("A(1x5)BC"), 7);
        assert_eq!(super::count("(3x3)XYZ"), 9);
        assert_eq!(super::count("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(super::count("(6x1)(1x3)A"), 6);
        assert_eq!(super::count("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn large() {
        assert_eq!(super::rec("(3x3)XYZ"), 9);
        assert_eq!(super::rec("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(super::rec("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(super::rec(
            "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
    }
}
