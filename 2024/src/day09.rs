
fn move_blocks(data: &str) -> usize {
    let count = data.len() / 2 + 1;
    let parse = |c: char| c.to_digit(10).unwrap() as usize;
    let mut left = data.chars();
    let mut right = (0..count).rev()
        .zip(data.chars().rev().step_by(2))
        .flat_map(|(k, v)| vec![k; parse(v)]);
    let total = data.chars().step_by(2).map(parse).sum::<usize>();

    let (mut pos, mut res) = (0, 0);
    'outer: for i in 0..count {
        let l = parse(left.next().unwrap());
        let r = parse(left.next().unwrap_or('0'));
        for j in 0..l.min(total - pos) {
            res += (pos + j) * i;
        }
        for j in 0..r {
            let k = right.next().unwrap();
            if k <= i { break 'outer; }
            res += (pos + l + j) * k;
        }
        pos += l + r;
    }
    res
}

fn move_files(data: &str) -> usize {
    let parse = |c: char| c.to_digit(10).unwrap() as usize;
    let files = data.chars().step_by(2).map(parse).collect::<Vec<_>>();
    let mut space = data[1..].chars().step_by(2).map(parse).collect::<Vec<_>>();
    let mut dist = files.iter().zip(space.iter())
        .scan(0, |a, (b, c)| { *a += b + c; Some(*a - c) })
        .collect::<Vec<_>>();

    let mut res = 0;
    for (k, &v) in files.iter().enumerate().rev() {
        let pos = match space.iter().position(|&sp| sp >= v) {
            Some(p) if p < k => { space[p] -= v; dist[p] += v; dist[p] - v },
            _ => if k > 0 {dist[k - 1] + space[k - 1]} else {0},
        };
        res += (0..v).map(|i| (pos + i) * k).sum::<usize>();
    }
    res
}

pub fn run(content: &str) {
    println!("{} {}", move_blocks(content), move_files(content));
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let score = super::move_blocks("2333133121414131402");
        assert_eq!(score, 1928);
    }

    #[test]
    fn large() {
        let score = super::move_files("2333133121414131402");
        assert_eq!(score, 2858);
    }
}
