
fn find_word(text: &str, word: &str) -> usize {
    let mut count = 0;
    let lines = text.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let pattern = word.as_bytes();
    for (i, line) in lines.iter().enumerate() {
        for j in 0..line.len() {
            for dx in -1..=1 { for dy in -1..=1 {
                if (0..word.len() as isize).all(|k| {
                    let (x, y) = (j as isize + k * dx, i as isize + k * dy);
                    y >= 0 && y < lines.len() as isize &&
                    x >= 0 && x < lines[y as usize].len() as isize &&
                    lines[y as usize][x as usize] == pattern[k as usize]
                }) { count += 1; }
            }}
        }
    }
    count
}

fn find_xmas(text: &str) -> usize {
    let mut count = 0;
    let lines = text.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let is_ms = |m, s| (m == b'M' && s == b'S') || (m == b'S' && s == b'M');
    for (i, line) in lines.iter().enumerate() {
        if i == 0 || i == lines.len() - 1 { continue; }
        for j in 1..line.len() - 1 {
            if lines[i][j] == b'A' &&
                is_ms(lines[i - 1][j - 1], lines[i + 1][j + 1]) &&
                is_ms(lines[i - 1][j + 1], lines[i + 1][j - 1]) { count += 1; }
        }
    }
    count
}

pub fn run(content: &str) {
    println!("{} {}", find_word(content, "XMAS"), find_xmas(content));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX";

    #[test]
    fn small() {
        assert_eq!(super::find_word(TEST, "XMAS"), 18);
    }

    #[test]
    fn large() {
        assert_eq!(super::find_xmas(TEST), 9);
    }
}
