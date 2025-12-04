use std::collections::HashSet;

type Point = (u32, u32);

struct Field {
    rolls: HashSet<Point>,
    bounds: Point,
}

impl Field {
    fn parse(text: &str) -> Self {
        let width = text.lines().next().unwrap().trim_end().len() as u32;
        let height = text.lines().count() as u32;
        let rolls = text.lines().enumerate().flat_map(|(y, s)| {
            s.chars().enumerate().filter_map(move |(x, c)| {
                if c == '@' {Some((x as u32, y as u32))} else {None}
            })
        }).collect::<HashSet<_>>();
        Self { rolls, bounds: (width, height) }
    }

    fn adjacent(&self, (x, y): Point) -> Vec<Point> {
        let mut pts = vec![];
        let (bx, by) = self.bounds;
        for ax in (if x > 0 {x - 1} else {0})..=(x + 1).min(bx - 1) {
            for ay in (if y > 0 {y - 1} else {0})..=(y + 1).min(by - 1) {
                if ax != x || ay != y { pts.push((ax, ay)); }
            }
        }
        pts
    }

    fn can_access(&self, pos: Point) -> bool {
        self.rolls.contains(&pos) &&
        self.adjacent(pos).into_iter()
            .filter(|p| self.rolls.contains(&p))
            .count() < 4
    }

    fn count_access(&self) -> usize {
        self.rolls.iter().filter(|&p| self.can_access(*p)).count()
    }

    fn clean_up(&mut self) -> usize {
        let mut removed = 0;
        loop {
            let to_remove = self.rolls.iter().copied()
                .filter(|&p| self.can_access(p))
                .collect::<HashSet<_>>();
            if to_remove.is_empty() { break; }
            removed += to_remove.len();
            self.rolls.retain(|p| !to_remove.contains(p));
        }
        removed
    }
}

pub fn run(content: &str) {
    let mut field = Field::parse(content);
    let res1 = field.count_access();
    let res2 = field.clean_up();
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.";

    #[test]
    fn small() {
        assert_eq!(super::Field::parse(TEST).count_access(), 13);
    }

    #[test]
    fn large() {
        assert_eq!(super::Field::parse(TEST).clean_up(), 43);
    }
}
