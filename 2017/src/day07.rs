use std::collections::{HashMap, HashSet};

struct Node {
    name: String,
    children: Vec<String>,
    weight: u32,
}

struct Tree {
    nodes: HashMap<String, Node>,
}

impl Node {
    fn parse(text: &str) -> Self {
        let (l, r) = text.split_once(" -> ").unwrap_or((text, ""));
        let (l1, l2) = l.trim_start().split_once(" ").unwrap();
        Node {
            name: l1.to_owned(),
            weight: l2.trim_matches(&['(', ')']).parse::<u32>().unwrap(),
            children: if r.is_empty() {vec![]} else
                {r.split(", ").map(|s| s.to_owned()).collect()},
        }
    }
}

impl Tree {
    fn parse(text: &str) -> Self {
        let nodes = text.trim().lines().map(Node::parse)
            .map(|x| (x.name.clone(), x))
            .collect::<HashMap<_, _>>();
        Tree { nodes }
    }

    fn find_root(&self) -> String {
        let s1 = self.nodes.values()
            .flat_map(|x| x.children.iter().cloned())
            .collect::<HashSet<_>>();
        let s2 = self.nodes.values()
            .filter(|x| !x.children.is_empty())
            .map(|x| x.name.clone())
            .collect::<HashSet<_>>();
        s2.difference(&s1).next().unwrap().to_owned()
    }

    fn calc_weight(&self, name: &str) -> u32 {
        let node = self.nodes.get(name).unwrap();
        node.weight + node.children.iter()
            .map(|x| self.calc_weight(x)).sum::<u32>()
    }

    fn find_outlier(&self, name: &str) -> Option<(String, u32)> {
        let node = self.nodes.get(name).unwrap();
        if node.children.len() < 3 { return None; }
        let mut items = node.children.iter()
            .map(|x| (self.calc_weight(x), x.clone()))
            .collect::<Vec<_>>();
        items.sort();
        let (a, b, (c, s)) = (items[0].0, items[1].0, items.pop().unwrap());
        if a == c { return None; }
        Some(if a == b {(s, a)} else {(items[0].1.clone(), c)})
    }

    fn find_last(&self, root: &str) -> u32 {
        let (mut key, mut val) = (root.to_owned(), 0);
        loop {
            match self.find_outlier(&key) {
                Some((k, v)) => (key, val) = (k, v),
                None => {
                    let node = self.nodes.get(&key).unwrap();
                    return node.weight + val - self.calc_weight(&key);
                },
            }
        }
    }
}

pub fn run(content: &str) {
    let tree = Tree::parse(content);
    let root = tree.find_root();
    println!("{} {}", root, tree.find_last(&root));
}

#[cfg(test)]
mod tests {
    const TEST: &str = r#"
        pbga (66)
        xhth (57)
        ebii (61)
        havc (66)
        ktlj (57)
        fwft (72) -> ktlj, cntj, xhth
        qoyq (66)
        padx (45) -> pbga, havc, qoyq
        tknk (41) -> ugml, padx, fwft
        jptl (61)
        ugml (68) -> gyxo, ebii, jptl
        gyxo (61)
        cntj (57)
    "#;

    #[test]
    fn small() {
        let tree = super::Tree::parse(TEST);
        assert_eq!(tree.find_root(), "tknk");
    }

    #[test]
    fn large() {
        let tree = super::Tree::parse(TEST);
        assert_eq!(tree.find_last(&tree.find_root()), 60);
    }
}
