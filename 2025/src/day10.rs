use good_lp::{Expression, ProblemVariables, SolverModel, Solution,
              solvers, variable};

#[derive(Clone, Debug)]
struct Machine {
    indicator: u64,
    buttons: Vec<u64>,
    joltage: Vec<u32>,
}

impl Machine {
    fn parse(text: &str) -> Self {
        let a = text.split(' ').collect::<Vec<_>>();
        let indicator = a[0].trim_matches(|c| c == '[' || c == ']')
            .chars().enumerate()
            .map(|(i, c)| if c == '#' {1_u64 << i} else {0})
            .sum::<u64>();
        let buttons = (1..a.len() - 1).map(|i| {
            a[i].trim_matches(|c| c == '(' || c == ')').split(',')
                .map(|s| 1_u64 << s.parse::<u32>().unwrap())
                .sum::<u64>()
        }).collect::<Vec<_>>();
        let joltage = a.last().unwrap()
            .trim_matches(|c| c == '{' || c == '}').split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Self { indicator, buttons, joltage }
    }

    fn min_press(&self) -> u32 {
        let limit = 1_i32 << self.buttons.len();
        (0..limit).filter_map(|i| {
            let res = self.buttons.iter().enumerate().filter_map(|(k, v)| {
                if (i & (1 << k)) != 0 {Some(v)} else {None}
            }).fold(0, |acc, val| acc ^ val);
            if res == self.indicator {Some(i.count_ones())} else {None}
        }).min().unwrap()
    }

    fn min_jolt(&self) -> Option<u32> {
        let mut problem = ProblemVariables::new();
        let vars = problem.add_vector(
            variable().integer().min(0), self.buttons.len());
        let objective: Expression = vars.iter().sum();

        let mut solution = problem.minimise(objective)
            .using(solvers::microlp::microlp);
        for (idx, val) in self.joltage.iter().enumerate() {
            let mut expr = Expression::from(0);
            for (k, v) in self.buttons.iter().enumerate() {
                if (*v & (1 << idx)) != 0 {
                    expr += vars[k];
                }
            }
            solution = solution.with(expr.eq(*val));
        }

        let result = solution.solve().ok()?;
        let total: f64 = vars.into_iter().map(|v| result.value(v)).sum();
        Some(total.round() as u32)
    }
}

pub fn run(content: &str) {
    let data = content.lines().map(Machine::parse).collect::<Vec<_>>();
    let res1 = data.iter().map(|x| x.min_press()).sum::<u32>();
    let res2 = data.iter().map(|x| x.min_jolt().unwrap()).sum::<u32>();
    println!("{res1} {res2}");
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn small() {
        let data = TEST.lines().map(super::Machine::parse).collect::<Vec<_>>();
        assert_eq!(data[0].min_press(), 2);
        assert_eq!(data[1].min_press(), 3);
        assert_eq!(data[2].min_press(), 2);
    }

    #[test]
    fn large() {
        let data = TEST.lines().map(super::Machine::parse).collect::<Vec<_>>();
        assert_eq!(data[0].min_jolt(), Some(10));
        assert_eq!(data[1].min_jolt(), Some(12));
        assert_eq!(data[2].min_jolt(), Some(11));
    }
}
