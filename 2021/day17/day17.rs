use std::ops::Range;

const X: (i32, i32) = (281, 311);
const Y: (i32, i32) = (-74, -54);

struct State {
    position: i32,
    speed: i32,
    steps: usize,
}

impl State {
    fn next(&mut self) {
        self.position += self.speed;
        self.speed -= 1;
        self.steps += 1;
    }
}

fn steps_x(speed: i32) -> Option<Range<usize>> {
    let mut start: Option<usize> = None;
    let mut state = State { position: 0, speed, steps: 0 };
    while state.position <= X.1 && state.speed > 0 {
        if state.position >= X.0 { start = start.or(Some(state.steps)); }
        state.next();
    }
    let unbounded = (X.0..=X.1).contains(&state.position);
    start.map(|value| value..if unbounded { usize::MAX } else { state.steps })
}

fn steps_y(speed: i32) -> Option<Range<usize>> {
    let mut start: Option<usize> = None;
    let mut state = if speed > 0 {
        State { position: 0, speed: -speed - 1, steps: (speed * 2 + 1) as usize }
    } else {
        State { position: 0, speed, steps: 0 }
    };
    while state.position >= Y.0 {
        if state.position <= Y.1 { start = start.or(Some(state.steps)); }
        state.next();
    }
    start.map(|value| value..state.steps)
}

fn combinations(ax: Vec<Range<usize>>, ay: Vec<Range<usize>>) -> usize {
    ay.iter().flat_map(|ry| ax.iter().map(move |rx|
        ry.clone().any(|y| rx.contains(&y)) as usize
    )).sum()
}

fn main() {
    let highest = (Y.0 + 1) * Y.0 / 2;
    let total = combinations(
        (1..=X.1).flat_map(steps_x).collect(),
        (Y.0..-Y.0).flat_map(steps_y).collect());
    println!("{} {}", highest, total);
}
