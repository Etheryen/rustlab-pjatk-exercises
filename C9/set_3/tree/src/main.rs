use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    children: Vec<Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
}

impl Node {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            children: Vec::new(),
            parent: Weak::new(),
        }))
    }

    fn print(node: &Rc<RefCell<Self>>) {
        Self::print_rec(node, 0);
    }

    fn padding(level: u8) -> String {
        (0..level * 2).map(|_| ' ').collect()
    }

    fn print_rec(node: &Rc<RefCell<Self>>, level: u8) {
        let data = node.borrow();

        let padding = Self::padding(level);

        match data.parent.upgrade() {
            Some(_) => println!("{padding}parent - EXISTS"),
            None => println!("{padding}patent - NONE"),
        }

        if data.children.is_empty() {
            println!("{padding}No children");
        }

        for (i, child) in data.children.iter().enumerate() {
            println!("{padding}child {}:", i);
            Self::print_rec(child, level + 1);
        }
    }

    fn add_child(parent: Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
        child.borrow_mut().parent = Rc::downgrade(&parent);
        parent.borrow_mut().children.push(Rc::clone(&child));
    }
}

fn main() {
    let top = Node::new();
    Node::add_child(Rc::clone(&top), Node::new());
    Node::add_child(Rc::clone(&top), Node::new());
    Node::add_child(Rc::clone(&top.borrow_mut().children[0]), Node::new());
    Node::add_child(Rc::clone(&top.borrow_mut().children[0]), Node::new());

    Node::print(&top);
}
