use std::{
    fs::File,
    io::{BufReader, BufRead},
    collections::HashMap,
    rc::Rc,
    cell::RefCell
};

use regex::Regex;

struct Directory {
    size: Option<u32>,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>
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

    fn parent(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Dir(d) => d.parent.as_ref().map(|parent| Rc::clone(parent)),
            _ => panic!("this node isn't dir")
        }
    }
}

pub fn solve(reader: BufReader<File>) {
    let root = build_tree(reader);
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

        if let Some(caps) = RE_CD.captures(&line) {
            let name = &caps[1];

            let child = cwd.borrow().get_child(name);
            if let Some(child) = child {
                cwd = child;
                continue;
            }

            let cwd_clone = Rc::clone(&cwd);
            let child = Rc::new(RefCell::new(Node::Dir(Directory{
                size: None,
                children: HashMap::new(),
                parent: Some(cwd_clone)
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

            let cwd_clone = Rc::clone(&cwd);
            let child = Rc::new(RefCell::new(Node::Dir(Directory{
                size: None,
                children: HashMap::new(),
                parent: Some(cwd_clone)
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

        if line == "$ cd .." {
            let parent = cwd.borrow().parent();
            if let Some(parent) = parent {
                cwd = parent;
                continue;
            }

            panic!("can't move to parent of current directory");
        }

        if line == "$ ls" {
            continue;
        }

        if line == "$ cd /" {
            cwd = Rc::clone(&root);
            continue;
        }

        panic!("line isn't matched, line: {}", line);
    }

    root
}
