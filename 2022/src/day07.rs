use itertools::Itertools;
use std::cell::RefCell;
use std::iter;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum NodeType {
    Directory(RefCell<Vec<Rc<Node>>>),
    File(usize),
}

#[derive(Debug)]
struct Node {
    name: String,
    type_: NodeType,
    parent: Weak<Node>,
}

impl Node {
    fn is_dir(&self) -> bool {
        matches!(self.type_, NodeType::Directory(_))
    }

    #[allow(dead_code)]
    fn path(&self) -> String {
        let mut arr: Vec<Rc<Node>> = Vec::new();
        let mut cur = self.parent.clone();
        while let Some(ptr) = cur.upgrade() {
            cur = ptr.parent.clone();
            arr.push(ptr);
        }
        arr.iter().rev().map(|x| &x.name)
            .chain(iter::once(&self.name)).join("/")
    }

    fn size(&self) -> usize {
        match &self.type_ {
            NodeType::File(size) => *size,
            NodeType::Directory(children) => {
                children.borrow().iter().map(|x| x.size()).sum()
            }
        }
    }
}

struct FileSystem {
    root: Rc<Node>,
}

impl FileSystem {
    fn from(input: &str) -> FileSystem {
        let root = Rc::new(Node {
            name: String::new(),
            type_: NodeType::Directory(RefCell::new(Vec::new())),
            parent: Weak::new(),
        });
        let mut current = root.clone();
        for line in input.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[..] {
                ["$", "cd", name] => current = match name {
                    "/" => root.clone(),
                    ".." => current.parent.upgrade().unwrap(),
                    _ => if let NodeType::Directory(children) = &current.type_ {
                        children.borrow().iter().filter(
                            |x| x.name == name).next().unwrap().clone()
                    } else {
                        panic!("Unexpected state")
                    }
                },
                ["$", "ls"] => (),
                [dir_or_size, name] => {
                    let node = Rc::new(Node {
                        name: String::from(name),
                        type_: if dir_or_size == "dir" {
                            NodeType::Directory(RefCell::new(Vec::new()))
                        } else {
                            NodeType::File(dir_or_size.parse().unwrap())
                        },
                        parent: Rc::downgrade(&current),
                    });
                    if let NodeType::Directory(children) = &current.type_ {
                        children.borrow_mut().push(node);
                    }
                },
                _ => panic!("Incorrect format")
            }
        }
        FileSystem { root }
    }

    fn iter(&self) -> FileSystemIterator {
        FileSystemIterator {
            cur: Rc::downgrade(&self.root),
            pos: vec![0],
        }
    }

    fn collect_dirs(&self) -> Vec<(Rc<Node>, usize)> {
        let mut result: Vec<(Rc<Node>, usize)> = Vec::new();
        for node in self.iter() {
            if node.is_dir() {
                let size = node.size();
                result.push((node, size));
            }
        }
        result
    }
}

struct FileSystemIterator {
    cur: Weak<Node>,
    pos: Vec<usize>,
}

impl Iterator for FileSystemIterator {
    type Item = Rc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.pos.pop() {
            let node = self.cur.upgrade().unwrap();
            if let NodeType::Directory(children) = &node.type_ {
                match children.borrow().get(index) {
                    Some(child) => {
                        self.pos.push(index + 1);
                        if child.is_dir() {
                            self.cur = Rc::downgrade(&child);
                            self.pos.push(0);
                        };
                        return Some(child.clone());
                    },
                    None => {
                        self.cur = node.parent.clone();
                    }
                }
            };
        }
        None
    }
}

pub fn run(content: &str) {
    let fs = FileSystem::from(content);
    let mut sizes: Vec<usize> = fs.collect_dirs().iter().map(|x| x.1).collect();
    sizes.sort();

    let small: usize = sizes.iter().filter(|&&x| x <= 100_000).sum();
    let unused = 70_000_000 - fs.root.size();
    let remove = sizes.into_iter().filter(|&x| x + unused >= 30_000_000).next();
    println!("{} {}", small, remove.unwrap());
}

#[cfg(test)]
mod tests {
    fn example() -> &'static str { r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#.trim()
    }

    #[test]
    pub fn dirs() {
        let fs = super::FileSystem::from(example());
        let sizes: Vec<usize> = fs.collect_dirs().iter().map(|x| x.1).collect();
        assert_eq!(sizes, vec![94853, 584, 24933642]);
    }
}
