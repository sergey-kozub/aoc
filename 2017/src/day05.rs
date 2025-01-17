
struct JumpSequence {
    offset: Vec<isize>,
    current: usize,
    limit: Option<isize>,
}

impl Iterator for JumpSequence {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let slot = &mut self.offset[self.current];
        let to = self.current as isize + *slot;
        match self.limit {
            Some(x) if *slot >= x => *slot -= 1,
            _ => *slot += 1,
        };
        if to < 0 || to >= self.offset.len() as isize { return None; }
        self.current = to as usize;
        Some(self.current)
    }
}

fn count_steps(offset: Vec<isize>, limit: Option<isize>) -> usize {
    let iter = JumpSequence { offset, current: 0, limit };
    iter.count() + 1
}

pub fn run(content: &str) {
    let data = content.lines()
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let v1 = count_steps(data.clone(), None);
    let v2 = count_steps(data, Some(3));
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let data = vec![0, 3, 0, 1, -3];
        assert_eq!(super::count_steps(data.clone(), None), 5);
        assert_eq!(super::count_steps(data, Some(3)), 10);
    }
}
