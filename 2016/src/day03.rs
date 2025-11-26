
fn parse(text: &str) -> Vec<[u32; 3]> {
    text.lines().map(|s| {
        s.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .try_into().unwrap()
    }).collect()
}

fn valid_triangle(sides: &[u32; 3]) -> bool {
    sides[0] + sides[1] > sides[2] &&
    sides[0] + sides[2] > sides[1] &&
    sides[1] + sides[2] > sides[0]
}

pub fn run(content: &str) {
    let data = parse(content);
    let triangles = data.iter().filter(|&x| valid_triangle(x)).count();
    let trans = (0..3).flat_map(|i| data.iter().map(move |&x| x[i]))
        .collect::<Vec<_>>();
    let triangles_v = trans.chunks(3)
        .filter(|&x| valid_triangle(x.try_into().unwrap())).count();
    println!("{} {}", triangles, triangles_v);
}
