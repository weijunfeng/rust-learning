use std::cell::RefCell;
use std::fmt::{Display, Pointer};
use std::rc::{Rc, Weak};

use crate::r#ref::{List, Node, Tail};

// use crate::r#box::List;

// mod r#box;
mod r#ref;

fn main() {
    // let list = List::from_array(&[1, 2, 3]);
    // println!("list:{list}")

    let a = Rc::new(List::Cons(5, RefCell::new(Tail::Rc(Rc::new(List::Nil)))));
    let b = Rc::new(List::Cons(10, RefCell::new(Tail::Weak(Rc::downgrade(&a)))));
    if let Some(tail) = a.tail() {
        *(tail.borrow_mut()) = Tail::Rc(Rc::clone(&b))
    }
    println!("{:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 12,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    println!(
        "leaf parent:{:?}",
        leaf.parent.try_borrow().map(|a| a.upgrade())
    );
    let branch = Rc::new(Node {
        value: 13,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
        "leaf parent:{:?}",
        leaf.parent.try_borrow().map(|a| a.upgrade())
    );
}
