use std::cell::RefCell;
use std::fmt;
use std::fs;
use std::ptr;
use std::rc::{Rc, Weak};

// Node types
type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
struct NodeSplit {
    left: NodeRef,
    right: NodeRef,
}

#[derive(Debug)]
enum Content {
    Split(NodeSplit),
    Value(u32),
}

#[derive(Debug)]
struct Node {
    parent: Weak<RefCell<Node>>,
    content: Content,
}

// Node implementation
impl Node {
    fn create(content: Content) -> NodeRef {
        let node = Node { parent: Weak::new(), content };
        Rc::new(RefCell::new(node))
    }

    fn create_int(value: u32) -> NodeRef {
        Node::create(Content::Value(value))
    }
    
    fn create_pair(left: NodeRef, right: NodeRef) -> NodeRef {
        let node = Node::create(Content::Split(NodeSplit {
            left: Rc::clone(&left),
            right: Rc::clone(&right),
        }));
        left.borrow_mut().parent = Rc::downgrade(&node);
        right.borrow_mut().parent = Rc::downgrade(&node);
        node
    }

    fn from(text: &str) -> Option<NodeRef> {
        let (node, size) = Node::parse(text.as_bytes())?;
        if size == text.len() { Some(node) } else { None }
    }

    fn parse(text: &[u8]) -> Option<(NodeRef, usize)> {
        match text.first()? {
            b'0'..=b'9' => {
                let value = (text[0] - b'0') as u32;
                Some((Node::create_int(value), 1))
            },
            b'[' => {
                let (nl, sl) = Node::parse(&text[1..])?;
                text.get(sl + 1).filter(|&ch| *ch == b',')?;
                let (nr, sr) = Node::parse(&text[sl + 2..])?;
                text.get(sl + sr + 2).filter(|&ch| *ch == b']')?;
                Some((Node::create_pair(nl, nr), sl + sr + 3))
            },
            _ => None,
        }
    }

    fn value(&self) -> Option<u32> {
        match &self.content {
            Content::Value(value) => Some(*value),
            _ => None,
        }
    }

    fn next(&self, from: &Node, rev: bool) -> Option<(NodeRef, i16)> {
        if let Content::Split(split) = &self.content {
            let from_left = ptr::eq(from, split.left.as_ptr());
            let from_right = ptr::eq(from, split.right.as_ptr());
            let downward = !from_left && !from_right;
            if !rev && from_left || rev && downward {
                return Some((Rc::clone(&split.right), 1));
            } else if rev && from_right || !rev && downward {
                return Some((Rc::clone(&split.left), 1));
            };
        }
        Some((self.parent.upgrade()?, -1))
    }

    fn replace(&self, other: NodeRef) {
        let parent = self.parent.upgrade().unwrap();
        other.borrow_mut().parent = Rc::downgrade(&parent);

        let update = &mut parent.borrow_mut();
        let split = match update.content {
            Content::Split(ref mut split) => split,
            _ => unreachable!(),
        };
        if ptr::eq(self, split.left.as_ptr()) {
            split.left = other;
        } else {
            split.right = other;
        }
    }

    fn explode(&self) {
        let add_value = |it: Option<(NodeRef, _)>, value: u32| {
            if let Some((node, _)) = it {
                let sum = node.borrow().value().unwrap() + value;
                node.borrow_mut().content = Content::Value(sum);
            }
        };

        if let Content::Split(split) = &self.content {
            add_value(
                NodeIter::from(&split.left).rev().next(),
                split.left.borrow().value().unwrap());
            add_value(
                NodeIter::from(&split.right).next(),
                split.right.borrow().value().unwrap());
            self.replace(Node::create_int(0));
        }
    }

    fn split(&self) {
        let value = self.value().unwrap();
        let left = Node::create_int(value / 2);
        let right = Node::create_int((value + 1) / 2);
        self.replace(Node::create_pair(left, right));
    }

    fn clone(&self) -> NodeRef {
        match &self.content {
            Content::Split(split) => Node::create_pair(
                split.left.borrow().clone(), split.right.borrow().clone()),
            Content::Value(value) => Node::create_int(*value),
        }
    }

    fn magnitude(&self) -> u32 {
        match &self.content {
            Content::Split(split) => 3 * split.left.borrow().magnitude() +
                                     2 * split.right.borrow().magnitude(),
            Content::Value(value) => *value,
        }
    }
}

// Node iteration
struct NodeIter {
    previous: NodeRef,
    current: NodeRef,
    level: i16,
}

impl NodeIter {
    fn from(node: &NodeRef) -> NodeIter {
        NodeIter {
            previous: Rc::clone(node),
            current: Rc::clone(node),
            level: 0,
        }
    }

    fn replace(&mut self, next: (NodeRef, i16)) -> bool {
        self.previous = Rc::clone(&self.current);
        self.current = next.0;
        self.level += next.1;
        matches!(self.current.borrow().content, Content::Value(_))
    }
}

impl Iterator for NodeIter {
    type Item = (NodeRef, i16);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = self.current.borrow().next(&self.previous.borrow(), false)?;
            if self.replace(node) { return Some((Rc::clone(&self.current), self.level)) }
        }
    }
}

impl DoubleEndedIterator for NodeIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let node = self.current.borrow().next(&self.previous.borrow(), true)?;
            if self.replace(node) { return Some((Rc::clone(&self.current), self.level)) }
        }
    }
}

// Node formatting
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.content {
            Content::Split(split) =>
                write!(f, "[{},{}]", split.left.borrow(), split.right.borrow()),
            Content::Value(value) => write!(f, "{}", value),
        }
    }
}

// Node functions
fn snail_add(a: NodeRef, b: NodeRef) -> NodeRef {
    let result = Node::create_pair(a, b);
    'outer: loop {
        for (node, level) in NodeIter::from(&result) {
            if level > 4 {
                let parent = node.borrow().parent.upgrade().unwrap();
                parent.borrow().explode();
                continue 'outer;
            }
        }
        for (node, _) in NodeIter::from(&result) {
            let value = node.borrow().value().unwrap();
            if value > 9 {
                node.borrow().split();
                continue 'outer;
            }
        }
        break;
    }
    result
}

fn main() {
    let input: Vec<NodeRef> = fs::read_to_string("input.txt").expect("Error reading input")
        .lines().map(|s| Node::from(s).unwrap()).collect();

    let sum = input.iter().fold(input[0].borrow().clone(), |a, b| {
        snail_add(a, b.borrow().clone())
    }).borrow().magnitude();
    let largest = input.iter().enumerate().flat_map(|(i, a)|
        input.iter().enumerate().map(move |(j, b)| {
            let result = snail_add(a.borrow().clone(), b.borrow().clone());
            if i != j { result.borrow().magnitude() } else { 0 }
        })
    ).max().unwrap();
    println!("{} {}", sum, largest)
}
