use std::cell::{RefCell};
use std::cmp::min;
use crate::fileio;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::from_utf8;

mod file_system {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    pub struct FileSystemObject {
        name: String,
        size: u32,
        update_size: bool,
        children: HashMap<String, Rc<RefCell<FileSystemObject>>>,
        is_dir: bool,
    }

    impl FileSystemObject {
        pub fn get_name(&self) -> &String {
            &self.name
        }

        pub fn get_size(&mut self) -> u32 {
            if self.update_size {
                self.size = 0;
                for (_key, child) in &self.children {
                    self.size += child.borrow_mut().get_size();
                }
                self.update_size = false;
            }
            return self.size;
        }

        pub fn get_children(&mut self) -> &mut HashMap<String, Rc<RefCell<FileSystemObject>>> {
            &mut self.children
        }

        pub fn new_file(name: &String, size: u32) -> FileSystemObject {
            return FileSystemObject {
                name: name.clone(),
                size,
                update_size: false,
                children: Default::default(),
                is_dir: false,
            };
        }

        pub fn new_dir(name: &String) -> FileSystemObject {
            return FileSystemObject {
                name: name.clone(),
                size: 0,
                update_size: true,
                children: Default::default(),
                is_dir: true,
            };
        }

        pub fn is_dir(&self) -> bool {
            return self.is_dir;
        }
    }
}

use file_system::*;

fn create_files() -> Rc<RefCell<FileSystemObject>> {
    let input = fileio::input("src/day7.txt");
    let root = Rc::new(RefCell::new(FileSystemObject::new_dir(&String::from("/"))));
    let mut path: Vec<Rc<RefCell<FileSystemObject>>> = Vec::new();
    path.push(Rc::clone(&root));
    for ln in input {
        let line = ln.as_bytes();
        if line[0] == '$' as u8 {
            if &line[2..4] == "cd".as_bytes() {
                let dest = &line[5..];
                if dest == "..".as_bytes() {
                    path.pop();
                } else if dest == "/".as_bytes() {
                    path.truncate(1);
                } else {
                    let dir_rc = Rc::clone(path.last_mut().unwrap());
                    path.push(Rc::clone(&dir_rc.borrow_mut().get_children()[from_utf8(dest).unwrap()]));
                }
            }
        } else {
            let dir_rc = Rc::clone(path.last_mut().unwrap());
            let mut dir = dir_rc.borrow_mut();
            if line.len() > 3 && &line[0..3] == "dir".as_bytes() {
                let name = String::from(from_utf8(&line[4..]).unwrap());
                dir.get_children().insert(name.clone(), Rc::new(RefCell::new(FileSystemObject::new_dir(&name))));
            } else {
                let parts: Vec<&str> = ln[..].splitn(2, " ").collect();
                let size = parts[0].parse::<u32>().unwrap();
                let name = String::from(parts[1]);
                dir.get_children().insert(name.clone(), Rc::new(RefCell::new(FileSystemObject::new_file(&name, size))));
            }
        }
    }
    return root;
}

fn print_files(root_rc: &Rc<RefCell<FileSystemObject>>) {
    fn pf_nested(node_rc: &Rc<RefCell<FileSystemObject>>, lvl: usize) {
        let mut node = node_rc.borrow_mut();
        println!("{}{} {}", " ".repeat(lvl*4), node.get_size(), node.get_name());
        for (_key, child) in node.get_children() {
            pf_nested(child, lvl+1);
        }
    }
    pf_nested(&root_rc, 0);
}

pub fn solve_a() {
    let mut root_rc = create_files();
    fn dfs(node_rc: &Rc<RefCell<FileSystemObject>>) -> u32 {
        let mut sum = 0;
        let mut node = node_rc.borrow_mut();
        for (_key, child) in node.get_children() {
            if child.borrow_mut().is_dir() {
                sum += dfs(child);
            }
        }
        sum + if node.get_size() <= 100000 {node.get_size()} else {0}
    }
    println!("{}", dfs(&root_rc));
}

pub fn solve_b() {
    let mut root_rc = create_files();
    let space_needed = 30000000 - (70000000 - root_rc.borrow_mut().get_size());
    fn dfs(space_needed: u32, node_rc: &Rc<RefCell<FileSystemObject>>) -> u32 {
        let mut out = u32::MAX;
        let mut node = node_rc.borrow_mut();
        for (_key, child) in node.get_children() {
            if child.borrow_mut().is_dir() {
                out = min(out, dfs(space_needed, child));
            }
        }
        if node.get_size() >= space_needed {min(out, node.get_size())} else {out}
    }
    println!("{}", dfs(space_needed, &root_rc));
}