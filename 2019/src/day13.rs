use crate::intcode::IntCode;
use std::collections::HashMap;
use std::fmt;
use std::str;

struct Arcade {
    cpu: IntCode,
    width: usize,
    height: usize,
    data: Vec<u8>,
}

fn convert(tile_id: i64) -> u8 {
    match tile_id {
        0 => b'.',
        1 => b'#',
        2 => b'o',
        3 => b'~',
        4 => b'*',
        _ => b'?',
    }
}

impl Arcade {
    fn from(program: &str) -> Arcade {
        let mut width = 0_usize;
        let mut height = 0_usize;
        let mut field = HashMap::<(usize, usize), i64>::new();
        let mut cpu = IntCode::from(program);

        while let Some(a) = cpu.wait_many(3) {
            let (x, y) = (a[0] as usize, a[1] as usize);
            if x >= width { width = x + 1; }
            if y >= height { height = y + 1; }
            field.insert((x, y), a[2]);
        }

        let mut data = vec![b'?'; width * height];
        for (k, v) in field.into_iter() {
            data[k.1 * width + k.0] = convert(v);
        }

        let mut cpu = IntCode::from(program);
        cpu.set(0, 2);
        Arcade { cpu, width, height, data }
    }

    fn predict(&self, end: i64) -> Option<i64> {
        let mut cpu = self.cpu.clone();
        cpu.input.push_back(0);
        while let Some(a) = cpu.wait_many(3) {
            if a[2] == 4 && a[1] == end {
                return Some(a[0]);
            }
            if cpu.input.is_empty() {
                cpu.input.push_back(0);
            }
        }
        None
    }

    fn play(&mut self) -> i64 {
        let mut score: i64 = 0;
        let mut paddle: (i64, i64) = (0, 0);
        let mut ball: (i64, i64) = (0, 0);

        while let Some(a) = self.cpu.wait_many(3) {
            if a[0] == -1 && a[1] == 0 {
                score = a[2];
                continue;
            }
            let (x, y) = (a[0] as usize, a[1] as usize);
            self.data[y * self.width + x] = convert(a[2]);
            if a[2] == 3 { paddle = (a[0], a[1]); }
            if a[2] == 4 { ball = (a[0], a[1]); }

            if a[2] != 0 && paddle.0 == ball.0 && paddle.1 == ball.1 + 1 {
                let dx = self.predict(ball.1).unwrap_or(ball.0) - paddle.0;
                let dir = if dx > 0 {1} else {-1};
                if a[2] == 3 { self.cpu.input.clear(); }
                for _ in 0..dx.abs() { self.cpu.input.push_back(dir); }
            }
            if self.cpu.input.is_empty() {
                self.cpu.input.push_back(0);
            }
        }
        score
    }
}

impl fmt::Debug for Arcade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = (0..self.height).map(|i| {
            str::from_utf8(&self.data[i * self.width .. (i + 1) * self.width])
        }).fold(String::new(), |a, b| a + b.unwrap() + "\n");
        write!(f, "{}", result)
    }
}

pub fn run(content: &str) {
    let mut arcade = Arcade::from(content);
    print!("{:?}", arcade);
    let blocks = arcade.data.iter().filter(|&&c| c == b'o').count();
    let score = arcade.play();
    println!("{} {}", blocks, score);
}
