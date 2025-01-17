use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Stream {
    Group(Vec<Stream>),
    Garbage(String),
}

impl Stream {
    fn from(text: &str) -> Self {
        Self::parse(&mut text.chars().peekable())
    }

    fn parse(chars: &mut Peekable<Chars>) -> Self {
        match chars.next().unwrap() {
            '{' => {
                let mut items = vec![];
                loop {
                    if !matches!(chars.peek(), Some('}')) {
                        items.push(Self::parse(chars));
                    }
                    match chars.next().unwrap() {
                        ',' => continue,
                        '}' => break Self::Group(items),
                        ch => panic!("unexpected {ch}"),
                    }
                }
            },
            '<' => {
                let mut bytes = vec![];
                loop {
                    match chars.next().unwrap() {
                        '!' => { chars.next().unwrap(); },
                        '>' => break,
                        ch => bytes.push(ch as u8),
                    }
                }
                Self::Garbage(String::from_utf8(bytes).unwrap())
            },
            ch => panic!("unexpected {ch}"),
        }
    }

    fn score(&self, depth: u32) -> u32 {
        match self {
            Stream::Group(items) =>
                depth + items.iter().map(|x| x.score(depth + 1)).sum::<u32>(),
            Stream::Garbage(_) => 0,
        }
    }

    fn garbage(&self) -> u32 {
        match self {
            Stream::Group(items) =>
                items.iter().map(|x| x.garbage()).sum::<u32>(),
            Stream::Garbage(s) => s.len() as u32,
        }
    }
}

pub fn run(content: &str) {
    let stream = Stream::from(content);
    println!("{} {}", stream.score(1), stream.garbage());
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let score = |s| super::Stream::from(s).score(1);
        assert_eq!(score("{}"), 1);
        assert_eq!(score("{{{}}}"), 6);
        assert_eq!(score("{{},{}}"), 5);
        assert_eq!(score("{{{},{},{{}}}}"), 16);
        assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn large() {
        let garbage = |s| super::Stream::from(s).garbage();
        assert_eq!(garbage("<>"), 0);
        assert_eq!(garbage("<random characters>"), 17);
        assert_eq!(garbage("<<<<>"), 3);
        assert_eq!(garbage("<{!>}>"), 2);
        assert_eq!(garbage("<!!>"), 0);
        assert_eq!(garbage("<!!!>>"), 0);
        assert_eq!(garbage("<{o'i!a,<{i<a>"), 10);
    }
}
