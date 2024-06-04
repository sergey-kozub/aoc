use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::fs;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq)]
enum NodeType {
    Start,
    End,
    Small,
    Large,
}

#[derive(Debug)]
struct Node {
    name: String,
    type_: NodeType,
    edges: RefCell<Vec<Weak<Node>>>,
}

impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: String::from(name),
            type_: if name == "start" { NodeType::Start }
                else if name == "end" { NodeType::End }
                else if name.chars().all(|c| c.is_lowercase()) { NodeType::Small }
                else { NodeType::Large },
            edges: RefCell::new(Vec::new()),
        }
    }

    fn visit_internal(&self, path: &mut HashSet<String>, allow: bool) -> u32 {
        self.edges.borrow().iter()
            .map(|weak| weak.upgrade().unwrap())
            .map(|dest| dest.visit(path, allow && dest.type_ != NodeType::Start))
            .sum()
    }

    fn visit(&self, path: &mut HashSet<String>, allow: bool) -> u32 {
        let visited = path.contains(&self.name);
        match self.type_ {
            NodeType::End => { 1 },
            NodeType::Large => self.visit_internal(path, allow),
            NodeType::Start | NodeType::Small if !visited || allow => {
                if !visited { path.insert(self.name.clone()); }
                let result = self.visit_internal(path, allow && !visited);
                if !visited { path.remove(&self.name); }
                result
            },
            _ => { 0 },
        }
    }
}

fn main() {
    let input: Vec<Vec<String>> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(|s| s.split('-').map(String::from).collect()).collect();
    let mut nodes: HashMap<String, Rc<Node>> = HashMap::new();
    for pair in &input {
        for name in pair {
            nodes.entry(String::from(name)).or_insert_with(
                || Rc::new(Node::new(name)));
        }
    }
    for pair in input {
        let a = nodes.get(&pair[0]).unwrap();
        let b = nodes.get(&pair[1]).unwrap();
        a.edges.borrow_mut().push(Rc::downgrade(&b));
        b.edges.borrow_mut().push(Rc::downgrade(&a));
    }

    let start = nodes.get("start").unwrap();
    let c1 = start.visit(&mut HashSet::<String>::new(), false);
    let c2 = start.visit(&mut HashSet::<String>::new(), true);
    println!("{} {}", c1, c2)
}
