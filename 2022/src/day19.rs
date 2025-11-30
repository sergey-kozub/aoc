use regex::Regex;
use std::collections::{HashSet, VecDeque};

type Ore = u16;
type Clay = u16;
type Obsidian = u16;
type Geode = u16;
type Resources = (Ore, Clay, Obsidian, Geode);

#[derive(Debug)]
struct State {
    step: u16,
    resources: Resources,
    robots: Resources,
}

#[derive(Debug)]
struct Blueprint {
    id: u16,
    ore_cost: Ore,
    clay_cost: Ore,
    obsidian_cost: (Ore, Clay),
    geode_cost: (Ore, Obsidian),
}

impl Blueprint {
    fn from(input: &str) -> Blueprint {
        let pattern = Regex::new("\
            Blueprint (\\d+): \
            Each ore robot costs (\\d+) ore. \
            Each clay robot costs (\\d+) ore. \
            Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
            Each geode robot costs (\\d+) ore and (\\d+) obsidian.\
        ").unwrap();
        let caps = pattern.captures(input).unwrap();
        let get = |i| caps.get(i).unwrap().as_str().parse::<u16>().unwrap();
        Blueprint {
            id: get(1),
            ore_cost: get(2),
            clay_cost: get(3),
            obsidian_cost: (get(4), get(5)),
            geode_cost: (get(6), get(7)),
        }
    }

    fn mine(&self, limit: u16) -> u16 {
        let ore_max = [self.ore_cost, self.clay_cost, self.obsidian_cost.0,
                       self.geode_cost.0].into_iter().max().unwrap();
        let mut queue: VecDeque<State> = VecDeque::from([
            State { step: 0, resources: (0, 0, 0, 0), robots: (1, 0, 0, 0) }
        ]);
        let mut visited: HashSet<(Resources, Resources)> = HashSet::new();
        let mut result: u16 = 0;

        while let Some(State {step, resources: r, robots: p}) = queue.pop_front() {
            if step >= limit { break; }
            let mut add_state = |resources, robots| {
                if !visited.contains(&(resources, robots)) {
                    queue.push_back(State { step: step + 1, resources, robots });
                    visited.insert((resources, robots));
                    if resources.3 > result { result = resources.3; }
                }
            };
            let x = (r.0 + p.0, r.1 + p.1, r.2 + p.2, r.3 + p.3);
            if r.0 <= ore_max {
                add_state(x, p);
            }
            if r.0 >= self.ore_cost {
                add_state((x.0 - self.ore_cost, x.1, x.2, x.3),
                          (p.0 + 1, p.1, p.2, p.3));
            }
            if r.0 >= self.clay_cost {
                add_state((x.0 - self.clay_cost, x.1, x.2, x.3),
                          (p.0, p.1 + 1, p.2, p.3));
            }
            if r.0 >= self.obsidian_cost.0 && r.1 >= self.obsidian_cost.1 {
                add_state((x.0 - self.obsidian_cost.0,
                           x.1 - self.obsidian_cost.1, x.2, x.3),
                          (p.0, p.1, p.2 + 1, p.3));
            }
            if r.0 >= self.geode_cost.0 && r.2 >= self.geode_cost.1 {
                add_state((x.0 - self.geode_cost.0, x.1,
                           x.2 - self.geode_cost.1, x.3),
                          (p.0, p.1, p.2, p.3 + 1));
            }
        }
        result
    }
}

pub fn run(content: &str) {
    let bps: Vec<Blueprint> = content.lines().map(Blueprint::from).collect();
    let score_1 = bps.iter().map(|x| x.id * x.mine(24)).sum::<u16>();
    let score_2 = bps[..3].iter().map(|x| x.mine(32)).reduce(|a, b| a * b).unwrap();
    println!("{} {}", score_1, score_2);
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn robots() {
        let bp = super::Blueprint::from("Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.");
        assert_eq!(bp.mine(24), 9);
    }
}
