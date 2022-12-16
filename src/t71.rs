use std::{
    fs::File,
    io::{BufReader, BufRead},
    collections::HashMap,
    rc::{Rc, Weak},
    cell::RefCell
};

// use crate::mem::mem_print;

use regex::Regex;

struct Directory {
    size: Option<u32>,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>
}

enum Node {
    File(u32),
    Dir(Directory)
}

impl Node {
    fn add_child(&mut self, name: String, child: Rc<RefCell<Node>>) {
        match self {
            Node::Dir(d) => d.children.insert(name, child),
            _ => panic!("this node isn't dir")
        };
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Dir(d) => d.children.get(name).map(|child| Rc::clone(child)),
            _ => panic!("this node isn't dir")
        }
    }

    fn parent(&self) -> Option<Weak<RefCell<Node>>> {
        match self {
            Node::Dir(d) => d.parent.as_ref()
                .map(|p| Weak::clone(p)),
            _ => panic!("this node isn't dir")
        }
    }

    fn size(&mut self, total: &mut u32) -> u32 {
        if let Node::File(s) = self {
            return *s;
        }

        if let Node::Dir(dir) = self {
            if let Some(s) = dir.size {
                return s;
            }

            let mut size = 0;
            for child in dir.children.values() {
                size += child.borrow_mut().size(total);
            }
            dir.size = Some(size);

            if size < 100000 {
                *total += size;
            }

            return size;
        }

        unreachable!();
    }
}

pub fn solve(reader: BufReader<File>) {
    // mem_print();
    let root = build_tree(reader);
    // mem_print();
    let mut total = 0;
    root.borrow_mut().size(&mut total);
    println!("{}", total);
    // drop(root);
    // mem_print();
}

fn build_tree(reader: BufReader<File>) -> Rc<RefCell<Node>> {
    lazy_static! {
        static ref RE_CD: Regex = Regex::new(r"^\$ cd (\S*)$").unwrap();
        static ref RE_DIR: Regex = Regex::new(r"^dir (\S*)$").unwrap();
        static ref RE_FILE: Regex = Regex::new(r"^(\d*) (\S*)$").unwrap();
    }

    let root = Rc::new(RefCell::new(Node::Dir(Directory{
        size: None,
        children: HashMap::new(),
        parent: None
    })));
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
            let name = &caps[1];

            let child = cwd.borrow().get_child(name);
            if let Some(child) = child {
                cwd = child;
                continue;
            }

            let child = Rc::new(RefCell::new(Node::Dir(Directory{
                size: None,
                children: HashMap::new(),
                parent: Some(Rc::downgrade(&cwd))
            })));

            let child_clone = Rc::clone(&child);
            cwd.borrow_mut().add_child(name.to_string(), child_clone);
            cwd = child;
            continue;
        }

        if let Some(caps) = RE_DIR.captures(&line) {
            let name = &caps[1];

            if cwd.borrow().get_child(name).is_some() {
                continue;
            }

            let child = Rc::new(RefCell::new(Node::Dir(Directory{
                size: None,
                children: HashMap::new(),
                parent: Some(Rc::downgrade(&cwd))
            })));
            cwd.borrow_mut().add_child(name.to_string(), child);
            continue;
        }

        if let Some(caps) = RE_FILE.captures(&line) {
            let size = &caps[1].parse().unwrap();
            let name = &caps[2];

            if cwd.borrow().get_child(name).is_some() {
                continue;
            }

            let child = Rc::new(RefCell::new(Node::File(*size)));
            cwd.borrow_mut().add_child(name.to_string(), child);
            continue;
        }

        panic!("line isn't matched, line: {}", line);
    }

    root
}
