//use colored::*;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct TreeNode {
    pub node_type: Option<String>,
    pub node_name: Option<String>,
    pub size: Option<u64>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
            node_type: None,
            node_name: None,
            size: Some(0),
            children: vec![],
            parent: None,
        };
    }
    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }
    pub fn print(&self) -> String {
        if let Some(value) = &self.node_name {
            if !&self.children.is_empty() {
                return value.to_owned()
                    + "("
                    + &self.size.unwrap().to_string()
                    + ")"
                    + &String::from("[")
                    + &self
                        .children
                        .iter()
                        .map(|tn| tn.borrow().print())
                        .collect::<Vec<String>>()
                        .join(",")
                    + "]";
            } else {
                return value.to_string();
            }
        } else {
            String::new()
        }
    }

    pub fn get_total(&self, limit: u64, acc: &mut Vec<u64>) {
        if let Some(_) = &self.node_name {
            if &self.node_type == &Some("dir".to_string()) && self.size.as_ref().unwrap() < &limit {
                acc.push(self.size.unwrap());
                //total += &self.size.unwrap();
            }
            if !&self.children.is_empty() {
                self.children
                    .iter()
                    .for_each(|child| child.borrow().get_total(limit, acc));
            }
        }
    }

    pub fn get_min_delete(&self, limit: u64, acc: &mut Vec<u64>) {
        if let Some(_) = &self.node_name {
            if &self.node_type == &Some("dir".to_string()) && self.size.as_ref().unwrap() > &limit {
                acc.push(self.size.unwrap());
                //total += &self.size.unwrap();
            }
            if !&self.children.is_empty() {
                self.children
                    .iter()
                    .for_each(|child| child.borrow().get_min_delete(limit, acc));
            }
        }
    }
    pub fn increase_size(&mut self, size: u64) {
        let new_size = &self.size.unwrap() + size;
        self.size = Some(new_size);
    }
}

pub fn init_tree(file: &str) -> Rc<RefCell<TreeNode>> {
    let file = File::open(file).expect("Error in reading file");

    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);
    current.borrow_mut().node_name = Some('/'.to_string());
    current.borrow_mut().node_type = Some("dir".to_string());

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let row = line.unwrap().trim().to_string();
        let row: Vec<&str> = row.split(" ").collect();

        //check command
        if row[0] == "$" {
            //get command type - cd , ls
            // ls has no arguments, len row must be 2
            if row.len() < 3 {
                // ls commands to list current dir
                //println! {"Current content TODO"}
            } else {
                let arg = row[2];
                // process cd
                // .. move to parent -> current = current.parent
                // / move to root -> current = root
                // 'name' .. create node 'name', -> current = name

                if arg == ".." {
                    let current_clone = Rc::clone(&current);
                    //println!("Moving up from {:?}", current_clone.borrow().node_name);
                    current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                } else if arg == "/" {
                    //println!("Moving to root from {:?}", current.borrow().node_name);
                    current = Rc::clone(&root);
                } else {
                    let current_clone = Rc::clone(&current);
                    let children = &current_clone.borrow().children;
                    let child = children
                        .iter()
                        .find(|&node| node.borrow().node_name == Some(arg.to_string()))
                        .unwrap();

                    // println!(
                    //"Moving to {:?} from {:?}",
                    //     child.borrow().node_name,
                    //    current_clone.borrow().node_name
                    // );
                    current = Rc::clone(&child);
                }
            }
        } else {
            let (obj_type, obj_name) = (row[0], row[1]);
            if obj_type == "dir" {
                let child = Rc::new(RefCell::new(TreeNode::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                    mut_child.node_name = Some(obj_name.to_string());
                    mut_child.node_type = Some("dir".to_string());
                }
                //println!(
                //    "Adding {:?} to {:?}",
                //    child.borrow().node_name,
                //    current.borrow().node_name
                // );
            } else {
                let child = Rc::new(RefCell::new(TreeNode::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                let size = obj_type.parse::<u64>().unwrap();
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                    mut_child.node_name = Some(obj_name.to_string());
                    mut_child.node_type = Some("file".to_string());

                    mut_child.size = Some(size);
                }

                current.borrow_mut().increase_size(size);

                let current_clone = Rc::clone(&current);
                let parent = current_clone.borrow();
                propagate(parent.parent.as_ref(), size);

                //println!(
                //"Adding {:?} to {:?}",
                //    child.borrow().node_name,
                //     current.borrow().node_name
                // );
            }
        }
    }
    return root;
}
fn propagate(parent: Option<&Rc<RefCell<TreeNode>>>, size: u64) {
    if let Some(p) = parent {
        let new_current = Rc::clone(p);
        let mut mut_new_current = new_current.borrow_mut();
        mut_new_current.increase_size(size);
        //let new_parent = new_current.borrow();
        propagate(mut_new_current.parent.as_ref(), size)
    }
}
