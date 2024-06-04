use std::fs;

#[derive(Clone, Debug)]
struct Board {
    size: usize,
    cells: Vec<i32>,
    marked: Vec<bool>,
}

impl Board {
    fn create(size: usize, text: &str) -> Board {
        let mut result = Board {
            size: size,
            cells: text.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap()).collect(),
            marked: Vec::new(),
        };
        assert_eq!(result.cells.len(), size * size);
        result.marked.resize(size * size, false);
        result
    }
    
    fn marked_row(&self, i: usize) -> bool {
        (0..self.size).all(|j| self.marked[i * self.size + j])
    }
    
    fn marked_col(&self, j: usize) -> bool {
        (0..self.size).all(|i| self.marked[i * self.size + j])
    }
    
    fn ready(&self) -> bool {
        (0..self.size).any(|x| self.marked_row(x) || self.marked_col(x))
    }
    
    fn update(&mut self, mark: i32) {
        if let Some(x) = self.cells.iter().position(|&x| x == mark) {
            self.marked[x] = true
        }
    }
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt").expect("Error reading input")
        .split("\n\n").map(String::from).collect();
    let numbers: Vec<i32> = input[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Board> = input.iter().skip(1).map(|s| Board::create(5, s)).collect();
    type BoardAt = (Board, i32);

    let mut winner: Option<BoardAt> = None;
    let mut loser: Option<BoardAt> = None;
    let mut index: Vec<usize> = (0..boards.len()).collect();
    for num in numbers {
        for board in &mut boards {
            board.update(num)
        };
        if winner.is_none() {
            if let Some(board) = boards.iter().find(|x| x.ready()) {
                winner = Some((board.clone(), num))
            }
        };
        if index.len() == 1 && boards[index[0]].ready() {
            loser = Some((boards[index[0]].clone(), num))
        };
        index.retain(|x| { !boards[*x].ready() });
    }

    let score = |x: &BoardAt| -> i32 {
        x.1 * x.0.cells.iter().zip(x.0.marked.iter())
            .map(|(x, m)| x * (!m) as i32).sum::<i32>()
    };
    println!("{} {}", score(&winner.unwrap()), score(&loser.unwrap()))
}
