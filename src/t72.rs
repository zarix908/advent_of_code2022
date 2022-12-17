use std::{
    fs::File,
    io::{BufReader, BufRead},
    collections::HashMap,
    rc::{Rc, Weak},
    cell::RefCell
};

use lazy_static::__Deref;
use regex::Regex;

struct Directory {
    size: Option<u32>,
    children: HashMap<Rc<String>, Rc<RefCell<Node>>>,
}

enum NodeEntity {
    File(u32),
    Dir(Directory)
}

struct Node {
    entity: NodeEntity,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn add_child(&mut self, name: Rc<String>, child: Rc<RefCell<Node>>) {
        match &mut self.entity {
            NodeEntity::Dir(d) => d.children.insert(name, child),
            _ => panic!("this node isn't dir")
        };
    }

    fn get_child(&self, name: &Rc<String>) -> Option<Rc<RefCell<Node>>> {
        match &self.entity {
            NodeEntity::Dir(d) => d.children.get(name)
                .map(|child| Rc::clone(child)),
            _ => panic!("this node isn't dir")
        }
    }

    fn parent(&self) -> Option<Weak<RefCell<Node>>> {
        self.parent.as_ref().map(|p| Weak::clone(p))
    }

    fn is_file(&self) -> bool {
        match self.entity {
            NodeEntity::File(_) => true,
            _ => false
        }
    }

    fn size(&self) -> Option<u32> {
        match &self.entity {
            NodeEntity::Dir(d) => d.size,
            NodeEntity::File(s) => Some(*s)
        }
    }

    fn set_size(&mut self, size: u32) {
        match &mut self.entity {
            NodeEntity::Dir(d) => d.size = Some(size),
            _ => panic!("can't set size to file")
        };
    }
}

struct NodeIter {
    stack: Vec<(Rc<String>, Rc<RefCell<Node>>)>
}

impl Iterator for NodeIter {
    type Item = (Rc<String>, Rc<RefCell<Node>>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.stack.pop() {
            match &item.1.borrow().deref().entity {
                NodeEntity::Dir(d) => {
                    for (name, child) in &d.children {
                        self.stack.push((Rc::clone(name), Rc::clone(child)));
                    }
                },
                _ => ()
            };

            return Some(item);
        }

        None
    }
}

struct Tree {
    root: Rc<RefCell<Node>>
}

impl IntoIterator for &Tree {
    type Item = (Rc<String>, Rc<RefCell<Node>>);
    type IntoIter = NodeIter;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter{stack: vec![(Rc::new(String::from("/")), Rc::clone(&self.root))]}
    }
}

pub fn solve(reader: BufReader<File>) {
    let tree = build_tree(reader);

    for node in tree.into_iter().map(|p| p.1) {
        if node.borrow().is_file() {
            let size = node.borrow().size().unwrap();
            let mut parent = node.borrow().parent();

            while let Some(current) = parent.as_ref().map(|n| Weak::clone(n)) {
                let current = current.upgrade().unwrap();
                let size = match current.borrow().size() {
                    Some(s) => s + size,
                    None => size
                };
                current.borrow_mut().set_size(size);
                parent = current.borrow().parent();
            };
        }
    }

    let need_free = tree.root.borrow().size().unwrap() - 40000000;
    let mut will_free = 70000000;
    for node in tree.into_iter().map(|p| p.1) { 
        if node.borrow().is_file() {
            continue;
        }

        let size = node.borrow().size().unwrap(); 
        if size > need_free && size < will_free {
            will_free = size;
        }
    }   

    println!("{}", will_free);
}

fn build_tree(reader: BufReader<File>) -> Tree {
    lazy_static! {
        static ref RE_CD: Regex = Regex::new(r"^\$ cd (\S*)$").unwrap();
        static ref RE_DIR: Regex = Regex::new(r"^dir (\S*)$").unwrap();
        static ref RE_FILE: Regex = Regex::new(r"^(\d*) (\S*)$").unwrap();
    }

    let root = Rc::new(RefCell::new(
        Node{
            entity: NodeEntity::Dir(Directory{
                size: None,
                children: HashMap::new(),
            }),
            parent: None
        }
    ));
    let mut cwd = Rc::clone(&root);

    for line in reader.lines() {
        let line = line.expect("read line failed");

        if line == "$ ls" {
            continue;
        }

        if line == "$ cd /" {
            cwd = Rc::clone(&root);
            continue;
        }

        if line == "$ cd .." {
            let parent = cwd.borrow().parent();
            if let Some(parent) = parent {
                // Rc точно существует, так как родительский элемент в дереве владеет дочерним
                cwd = parent.upgrade().unwrap();
                continue;
            }

            panic!("can't move to parent of current directory");
        }

        if let Some(caps) = RE_CD.captures(&line) {
            let name = Rc::new(caps[1].to_string());

            let child = cwd.borrow().get_child(&name);
            if let Some(child) = child {
                cwd = child;
                continue;
            }

            let child = Rc::new(RefCell::new(
                Node{
                    entity: NodeEntity::Dir(Directory{
                        size: None,
                        children: HashMap::new(),
                    }),
                    parent: Some(Rc::downgrade(&cwd))
                }
            ));
            let child_clone = Rc::clone(&child);
            cwd.borrow_mut().add_child(name, child_clone);
            cwd = child;
            continue;
        }

        if let Some(caps) = RE_DIR.captures(&line) {
            let name = Rc::new(caps[1].to_string());

            if cwd.borrow().get_child(&name).is_some() {
                continue;
            }

            let child = Rc::new(RefCell::new(
                Node{
                    entity: NodeEntity::Dir(Directory{
                        size: None,
                        children: HashMap::new()
                    }),
                    parent: Some(Rc::downgrade(&cwd))
                }
            ));
            cwd.borrow_mut().add_child(name, child);
            continue;
        }

        if let Some(caps) = RE_FILE.captures(&line) {
            let size = &caps[1].parse().unwrap();
            let name = Rc::new(caps[2].to_string());

            if cwd.borrow().get_child(&name).is_some() {
                continue;
            }

            let child = Rc::new(RefCell::new(
                Node{
                    entity: NodeEntity::File(*size),
                    parent: Some(Rc::downgrade(&cwd))
                }
            ));
            cwd.borrow_mut().add_child(name, child);
            continue;
        }

        panic!("line isn't matched, line: {}", line);
    }

    Tree{root}
}
