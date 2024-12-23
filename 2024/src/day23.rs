use std::collections::{HashMap, HashSet};

struct Graph(HashMap<String, HashSet<String>>);
struct Clique(HashSet<String>);

impl Graph {
    fn parse(text: &str) -> Self {
        let mut data = HashMap::new();
        for line in text.lines() {
            let (l, r) = line.split_once('-').unwrap();
            data.entry(l.to_owned()).or_insert_with(HashSet::new)
                .insert(r.to_owned());
            data.entry(r.to_owned()).or_insert_with(HashSet::new)
                .insert(l.to_owned());
        }
        Self(data)
    }

    fn tuples(&self) -> Vec<Clique> {
        let mut result = vec![];
        for (node, conn) in self.0.iter() {
            for other in conn.iter() {
                if node < other {
                    let a = [node.clone(), other.clone()];
                    result.push(Clique(HashSet::from(a)));
                }
            }
        }
        result
    }

    fn extend(&self, data: Vec<Clique>) -> Vec<Clique> {
        let mut result = vec![];
        let mut visited = HashSet::new();
        for item in data {
            for (node, conn) in self.0.iter() {
                if item.0.contains(node) { continue; }
                if item.0.iter().all(|x| conn.contains(x)) {
                    let mut a = Vec::from_iter(item.0.iter().cloned());
                    a.push(node.clone());
                    a.sort();
                    if visited.insert(a.join("")) {
                        let next = HashSet::from_iter(a.into_iter());
                        result.push(Clique(next));
                    }
                }
            }
        }
        result
    }

    fn password(&self) -> String {
        let mut data = self.tuples();
        while data.len() > 1 {
            data = self.extend(data);
            // println!("{}: {}", data[0].0.len(), data.len());
        }
        let mut a = Vec::from_iter(data.pop().unwrap().0.into_iter());
        a.sort();
        a.join(",")
    }
}

pub fn run(content: &str) {
    let graph = Graph::parse(content);
    let v1 = graph.extend(graph.tuples()).into_iter()
        .filter(|a| a.0.iter().any(|s| s.starts_with('t'))).count();
    println!("{} {}", v1, graph.password());
}

#[cfg(test)]
mod tests {
    const TEST: &str = "\
        kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\n\
        wh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\n\
        td-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\n\
        kh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn";

    #[test]
    fn small() {
        let graph = super::Graph::parse(TEST);
        let triple = graph.extend(graph.tuples());
        assert_eq!(triple.len(), 12);
    }

    #[test]
    fn large() {
        let graph = super::Graph::parse(TEST);
        assert_eq!(graph.password(), "co,de,ka,ta");
    }
}
