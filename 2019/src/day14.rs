use std::cmp;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Quantity(u64, String);

impl Quantity {
    fn from(text: &str) -> Quantity {
        let parts: Vec<&str> = text.split(' ').collect();
        Quantity(parts[0].parse::<u64>().unwrap(), String::from(parts[1]))
    }
}

#[derive(Clone, Debug)]
struct Formula {
    source: Vec<Quantity>,
    result: Quantity,
}

impl Formula {
    fn from(text: &str) -> Formula {
        let parts: Vec<&str> = text.split(" => ").collect();
        Formula {
            source: parts[0].split(", ").map(Quantity::from).collect(),
            result: Quantity::from(parts[1]),
        }
    }
}

#[derive(Debug)]
struct Reactions(HashMap<String, Formula>);

impl Reactions {
    fn from(text: &str) -> Reactions {
        let mut result: HashMap<String, Formula> = HashMap::new();
        for f in text.lines().map(Formula::from) {
            result.insert(f.result.1.clone(), f);
        }
        Reactions(result)
    }

    fn calc(&self, produce: Quantity, spare: &mut HashMap<String, u64>) -> u64 {
        if produce.1 == "ORE" {
            produce.0
        } else {
            let mut qty = produce.0;
            if let Some(existing) = spare.get_mut(&produce.1) {
                let reduce = cmp::min(*existing, qty);
                *existing -= reduce;
                qty -= reduce;
            }
            if qty > 0 {
                let f = self.0.get(&produce.1).unwrap();
                let d = f.result.0;
                let mut m = qty / d;
                if qty % d != 0 {
                    *spare.entry(produce.1).or_default() += d - qty % d;
                    m += 1;
                }
                f.source.iter().map(|q| {
                    self.calc(Quantity(q.0 * m, q.1.clone()), spare)
                }).sum()
            } else {
                0
            }
        }
    }

    fn calc_fuel(&self, n: u64) -> u64 {
        self.calc(Quantity(n, "FUEL".to_string()), &mut HashMap::new())
    }

    fn calc_max(&self, n: u64) -> u64 {
        let (mut l, mut r) = (1, n);
        while l < r - 1 {
            let m = (l + r) / 2;
            let v = self.calc_fuel(m);
            if v < n { l = m; } else { r = m; }
        }
        l
    }
}

pub fn run(content: &str) {
    let inst = Reactions::from(content);
    println!("{} {}", inst.calc_fuel(1), inst.calc_max(1_000_000_000_000));
}

#[cfg(test)]
mod tests {
    fn example1() -> super::Reactions {
        super::Reactions::from("\
            157 ORE => 5 NZVS\n\
            165 ORE => 6 DCFZ\n\
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
            179 ORE => 7 PSHF\n\
            177 ORE => 5 HKGWZ\n\
            7 DCFZ, 7 PSHF => 2 XJWVT\n\
            165 ORE => 2 GPVTF\n\
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")
    }

    fn example2() -> super::Reactions {
        super::Reactions::from("\
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
            17 NVRVD, 3 JNWZP => 8 VPVL\n\
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
            22 VJHF, 37 MNCFX => 5 FWMGM\n\
            139 ORE => 4 NVRVD\n\
            144 ORE => 7 JNWZP\n\
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
            145 ORE => 6 MNCFX\n\
            1 NVRVD => 8 CXFTF\n\
            1 VJHF, 6 MNCFX => 4 RFSQX\n\
            176 ORE => 6 VJHF")
    }

    fn example3() -> super::Reactions {
        super::Reactions::from("\
            171 ORE => 8 CNZTR\n\
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
            114 ORE => 4 BHXH\n\
            14 VRPVC => 6 BMBT\n\
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
            5 BMBT => 4 WPTQ\n\
            189 ORE => 9 KTJDG\n\
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
            12 VRPVC, 27 CNZTR => 2 XDBXC\n\
            15 KTJDG, 12 BHXH => 5 XCVML\n\
            3 BHXH, 2 VRPVC => 7 MZWV\n\
            121 ORE => 7 VRPVC\n\
            7 XCVML => 6 RJRHP\n\
            5 BHXH, 4 VRPVC => 5 LTCX")
    }

    #[test]
    fn part1() {
        assert_eq!(13312, example1().calc_fuel(1));
        assert_eq!(180697, example2().calc_fuel(1));
        assert_eq!(2210736, example3().calc_fuel(1));
    }

    #[test]
    fn part2() {
        assert_eq!(82892753, example1().calc_max(1_000_000_000_000));
        assert_eq!(5586022, example2().calc_max(1_000_000_000_000));
        assert_eq!(460664, example3().calc_max(1_000_000_000_000));
    }
}
