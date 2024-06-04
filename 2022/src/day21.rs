use std::collections::HashMap;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    if n == 0 { return m; }
    while m != 0 {
        if m < n { (m, n) = (n, m); }
        m %= n;
    }
    n
}

#[derive(Debug)]
enum Monkey {
    Num(i64),
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
}

impl Monkey {
    fn from(input: &str) -> Monkey {
        let parts: Vec<&str> = input.split(" ").collect();
        let id = Monkey::id;
        match parts[..] {
            [n] => Monkey::Num(n.parse::<i64>().unwrap()),
            [a, "+", b] => Monkey::Add(id(a), id(b)),
            [a, "-", b] => Monkey::Sub(id(a), id(b)),
            [a, "*", b] => Monkey::Mul(id(a), id(b)),
            [a, "/", b] => Monkey::Div(id(a), id(b)),
            _ => panic!("Incorrect input")
        }
    }

    fn id(s: &str) -> u32 {
        u32::from_str_radix(s, 36).unwrap()
    }
}

#[derive(Debug)]
struct Banter {
    rules: HashMap<u32, Monkey>,
}

#[derive(Debug, PartialEq)]
struct Expr {
    var: i64,
    num: i64,
    den: i64,
}

impl Banter {
    fn from(input: &str) -> Banter {
        let iter = input.lines().map(|s| {
            let a: Vec<&str> = s.split(": ").collect();
            (Monkey::id(a[0]), Monkey::from(a[1]))
        });
        Banter {
            rules: HashMap::from_iter(iter),
        }
    }

    fn eval(&self, node: u32) -> i64 {
        match *self.rules.get(&node).unwrap() {
            Monkey::Num(n) => n,
            Monkey::Add(a, b) => self.eval(a) + self.eval(b),
            Monkey::Sub(a, b) => self.eval(a) - self.eval(b),
            Monkey::Mul(a, b) => self.eval(a) * self.eval(b),
            Monkey::Div(a, b) => self.eval(a) / self.eval(b),
        }
    }

    fn eval_var(&self, node: u32, var: u32) -> Expr {
        if node == var {
            return Expr { var: 1, num: 0, den: 1 };
        }
        let res = match *self.rules.get(&node).unwrap() {
            Monkey::Num(n) => Expr { var: 0, num: n, den: 1 },
            Monkey::Add(a, b) => {
                let [x, y] = [self.eval_var(a, var), self.eval_var(b, var)];
                Expr { var: x.var * y.den + y.var * x.den,
                       num: x.num * y.den + y.num * x.den,
                       den: x.den * y.den }
            },
            Monkey::Sub(a, b) => {
                let [x, y] = [self.eval_var(a, var), self.eval_var(b, var)];
                Expr { var: x.var * y.den - y.var * x.den,
                       num: x.num * y.den - y.num * x.den,
                       den: x.den * y.den }
            },
            Monkey::Mul(a, b) => {
                let [x, y] = [self.eval_var(a, var), self.eval_var(b, var)];
                assert!(x.var == 0 || y.var == 0);
                Expr { var: x.var * y.num + y.var * x.num,
                       num: x.num * y.num,
                       den: x.den * y.den }
            },
            Monkey::Div(a, b) => {
                let [x, y] = [self.eval_var(a, var), self.eval_var(b, var)];
                assert!(y.var == 0 && y.num != 0);
                Expr { var: x.var * y.den,
                       num: x.num * y.den,
                       den: x.den * y.num }
            },
        };
        let u = gcd(res.var.abs() as u64, res.num.abs() as u64);
        let v = gcd(u, res.den.abs() as u64) as i64;
        Expr { var: res.var / v, num: res.num / v, den: res.den / v }
    }

    fn solve(&self, node: u32, var: u32) -> i64 {
        let root = self.rules.get(&node).unwrap();
        if let Monkey::Add(a, b) = *root {
            let [x, y] = [self.eval_var(a, var), self.eval_var(b, var)];
            (x.num * y.den - y.num * x.den) / (y.var * x.den - x.var * y.den)
        } else {
            panic!();
        }
    }
}

pub fn run(content: &str) {
    let inst = Banter::from(content);
    let root = Monkey::id("root");
    let humn = Monkey::id("humn");
    println!("{} {}", inst.eval(root), inst.solve(root, humn));
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#.trim()
    }

    #[test]
    pub fn monkeys() {
        let inst = super::Banter::from(example());
        let root = super::Monkey::id("root");
        let humn = super::Monkey::id("humn");
        assert_eq!(inst.eval(root), 152);
        assert_eq!(inst.solve(root, humn), 301);
    }
}
