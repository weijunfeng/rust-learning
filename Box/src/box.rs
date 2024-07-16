use std::fmt::{Display, Formatter};

pub enum List {
    Con(i32, Box<List>),
    Nil,
}

impl List {
    pub fn from_array(array: &[i32]) -> List {
        if let [header, tail @ ..] = array {
            List::Con(*header, Box::new(List::from_array(tail)))
        } else {
            List::Nil
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            List::Con(_, _) => false,
            List::Nil => true,
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Con(header, tail) => {
                write!(f, "{header}{}", if tail.is_empty() { "" } else { ", " })
                    .and(tail.as_ref().fmt(f))
            }
            List::Nil => Ok(()),
        }
    }
}
