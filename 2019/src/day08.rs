
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn solve_1(data: &[u8], width: usize, height: usize) -> usize {
    let size = width * height;
    let layer = |n| &data[n * size .. (n + 1) * size];
    let count = |n, c| layer(n).iter().filter(|&x| *x == c).count();
    let min_0 = (0..data.len() / size).map(|n| (count(n, b'0'), n)).min().unwrap();
    count(min_0.1, b'1') * count(min_0.1, b'2')
}

fn solve_2(data: &[u8], width: usize, height: usize) -> String {
    let size = width * height;
    (0..height).map(|i| {
        (0..width).map(move |j| {
            (0..100).find_map(|k| {
                match data[k * size + i * width + j] {
                    b'0' => Some(' '),
                    b'1' => Some('x'),
                    b'2' => None,
                    _ => panic!(),
                }
            }).unwrap()
        }).collect::<String>()
    }).fold(String::new(), |a, b| a + &b + "\n")
}

pub fn run(content: &str) {
    let bytes = content.trim_end().as_bytes();
    println!("{}\n{}", solve_1(bytes, WIDTH, HEIGHT), solve_2(bytes, WIDTH, HEIGHT))
}

#[cfg(test)]
mod tests {
    #[test]
    fn image() {
        assert_eq!(super::solve_2("0222112222120000".as_bytes(), 2, 2), " x\nx \n");
    }
}
