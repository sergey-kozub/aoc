
const KEYPAD: [[i32; 4]; 9] = [
    [1, 2, 4, 1],  // 1
    [2, 3, 5, 1],  // 2
    [3, 3, 6, 2],  // 3
    [1, 5, 7, 4],  // 4
    [2, 6, 8, 4],  // 5
    [3, 6, 9, 5],  // 6
    [4, 8, 7, 7],  // 7
    [5, 9, 8, 7],  // 8
    [6, 9, 9, 8],  // 9
];

const FANCY_KEYPAD: [[i32; 4]; 13] = [
    [1, 1, 3, 1],   // 1
    [2, 3, 6, 2],   // 2
    [1, 4, 7, 2],   // 3
    [4, 4, 8, 3],   // 4
    [5, 6, 5, 5],   // 5
    [2, 7, 10, 5],  // 6
    [3, 8, 11, 6],  // 7
    [4, 9, 12, 7],  // 8
    [9, 9, 9, 8],   // 9
    [6, 11, 10, 10],   // A
    [7, 12, 13, 10],   // B
    [8, 12, 12, 11],   // C
    [11, 13, 13, 13],  // D
];

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn ordinal(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

fn parse(text: &str) -> Vec<Vec<Direction>> {
    text.lines().map(|line| {
        line.chars().map(|ch| match ch {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!(),
        }).collect()
    }).collect()
}

fn get_code(text: &str) -> String {
    let mut result = String::new();
    parse(text).into_iter().fold(5, |start, moves| {
        let end = moves.into_iter().fold(start,
            |pos, dir| KEYPAD[pos as usize - 1][dir.ordinal()]);
        result.push_str(&end.to_string());
        end
    });
    result
}

fn get_fancy_code(text: &str) -> String {
    let mut result = String::new();
    parse(text).into_iter().fold(5, |start, moves| {
        let end = moves.into_iter().fold(start,
            |pos, dir| FANCY_KEYPAD[pos as usize - 1][dir.ordinal()]);
        result.push_str(&format!("{:X}", end));
        end
    });
    result
}

pub fn run(content: &str) {
    println!("{} {}", get_code(content), get_fancy_code(content));
}

#[cfg(test)]
mod tests {
    const TEST: &str = "ULL\nRRDDD\nLURDL\nUUUUD";

    #[test]
    fn small() {
        assert_eq!(super::get_code(TEST), "1985");
    }

    #[test]
    fn large() {
        assert_eq!(super::get_fancy_code(TEST), "5DB3");
    }
}
