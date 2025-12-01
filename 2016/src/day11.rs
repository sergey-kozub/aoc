use regex::Regex;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum ItemType { Generator, Microchip }

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Item {
    name: String,
    type_: ItemType,
    floor: usize,
}

#[derive(Clone, Debug)]
struct State {
    floors: Vec<HashSet<Item>>,
    lift: usize,
}

impl State {
    fn find_moves(&self) -> Vec<Vec<Item>> {
        let items = &self.floors[self.lift];
        let mut moves = vec![];
        let mut generators = 0;
        for item1 in items {
            moves.push(vec![item1.clone()]);
            for item2 in items {
                if item1.type_ == item2.type_ && item1.name < item2.name {
                    moves.push(vec![item1.clone(), item2.clone()]);
                }
                if item1.type_ == ItemType::Generator && 
                    item2.type_ == ItemType::Microchip && 
                    item2.name == item1.name {
                    moves.push(vec![item1.clone(), item2.clone()]);
                }
            }
            if item1.type_ == ItemType::Generator {
                generators += 1;
            }
        }
        let has_chip = |name: &str| items.iter().any(
            |x| x.type_ == ItemType::Microchip && x.name == name);
        moves.retain(|m| {
            let all_gen = m.iter().all(|x| x.type_ == ItemType::Generator);
            let any_chip_at_risk = all_gen && generators > m.len() &&
                m.iter().any(|x| has_chip(&x.name));
            !any_chip_at_risk
        });
        moves
    }

    fn can_move_to(&self, content: &[Item], floor: usize) -> bool {
        let items = &self.floors[floor];
        let generators = items.iter().chain(content.iter())
            .filter_map(|x| if x.type_ == ItemType::Generator
                        {Some(x.name.clone())} else {None})
            .collect::<Vec<_>>();
        let microchips = items.iter().chain(content.iter())
            .filter_map(|x| if x.type_ == ItemType::Microchip
                        {Some(x.name.clone())} else {None})
            .collect::<Vec<_>>();
        let at_risk = !generators.is_empty() &&
            microchips.iter().any(|s| !generators.contains(s));
        !at_risk
    }

    fn move_to(&self, content: &[Item], floor: usize) -> Self {
        let mut floors = self.floors.clone();
        for key in content {
            if let Some(mut item) = floors[key.floor].take(key) {
                item.floor = floor;
                floors[floor].insert(item);
            } else {panic!()}
        }
        Self { floors, lift: floor }
    }

    fn search(&self) -> Option<usize> {
        let mut queue = VecDeque::from([(0_usize, self.clone())]);
        let mut visited = HashSet::from([self.hash_string()]);
        let last = self.floors.len() - 1;
        while let Some((step, state)) = queue.pop_front() {
            if state.floors.iter().take(last).all(|x| x.is_empty()) {
                return Some(step);
            }
            let mut next = vec![];
            if state.lift > 0 { next.push(state.lift - 1); }
            if state.lift < last { next.push(state.lift + 1); }
            for move_ in state.find_moves() {
                for floor in &next {
                    if state.can_move_to(&move_, *floor) {
                        let new_state = state.move_to(&move_, *floor);
                        let new_hash = new_state.hash_string();
                        if !visited.contains(&new_hash) {
                            queue.push_back((step + 1, new_state));
                            visited.insert(new_hash);
                        }
                    }
                }
            }
        }
        None
    }

    fn hash_string(&self) -> String {
        let mut materials = self.floors.iter().flat_map(|items| {
            items.iter().filter_map(|item| {
                if item.type_ == ItemType::Generator {Some(item.name.clone())}
                else {None}
            })
        }).collect::<Vec<_>>();
        materials.sort();
        let lines = self.floors.iter().enumerate().rev().map(|(k, v)| {
            let parts = [format!("F{} {} ", k + 1,
                         if self.lift == k {"E"} else {"."})].into_iter()
            .chain(materials.iter().flat_map(|name| {
                let id = name.chars().take(2).collect::<String>();
                let item1 = Item {
                    name: name.into(), type_: ItemType::Generator, floor: k };
                let item2 = Item {
                    name: name.into(), type_: ItemType::Microchip, floor: k };
                [if v.contains(&item1) {format!("{id}G")} else {" . ".into()},
                 if v.contains(&item2) {format!("{id}M")} else {" . ".into()}]
            })).collect::<Vec<_>>();
            parts.join(" ")
        }).collect::<Vec<_>>();
        lines.join("\n")
    }
}

impl State {
    fn parse(text: &str) -> Self {
        let re_generator = Regex::new(r"a (\w+) generator").unwrap();
        let re_microchip = Regex::new(r"a (\w+)-compatible microchip").unwrap();
        let floors = text.lines().enumerate().map(|(floor, line)| {
            let mut items = HashSet::<Item>::new();
            for cap in re_generator.captures_iter(line) {
                let name = cap.get(1).unwrap().as_str().into();
                items.insert(Item { name, type_: ItemType::Generator, floor });
            }
            for cap in re_microchip.captures_iter(line) {
                let name = cap.get(1).unwrap().as_str().into();
                items.insert(Item { name, type_: ItemType::Microchip, floor });
            }
            items
        }).collect::<Vec<_>>();
        Self { floors, lift: 0 }
    }
}

pub fn run(content: &str) {
    let mut state = State::parse(content);
    let part1 = state.search().unwrap();

    let generator = |name| Item { name, type_: ItemType::Generator, floor: 0 };
    let microchip = |name| Item { name, type_: ItemType::Microchip, floor: 0 };
    state.floors[0].insert(generator("elerium".into()));
    state.floors[0].insert(microchip("elerium".into()));
    state.floors[0].insert(generator("dilithium".into()));
    state.floors[0].insert(microchip("dilithium".into()));
    let part2 = state.search().unwrap();
    println!("{} {}", part1, part2);
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        The first floor contains a hydrogen-compatible microchip \
            and a lithium-compatible microchip.\n\
        The second floor contains a hydrogen generator.\n\
        The third floor contains a lithium generator.\n\
        The fourth floor contains nothing relevant.";

    #[test]
    fn small() {
        let state = super::State::parse(TEST);
        assert_eq!(state.search(), Some(11));
    }
}
