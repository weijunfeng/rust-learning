use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub enum Tail {
    Rc(Rc<List>),
    Weak(Weak<List>),
}

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Tail>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Tail>> {
        match self {
            List::Cons(_, tail) => Some(tail),
            List::Nil => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub children: RefCell<Vec<Rc<Node>>>,
    // 使用Weak<T>解决循环引用问题
    // 使用RefCell<T> 不可变引用时也可改变内部数据
    pub parent: RefCell<Weak<Node>>,
}
