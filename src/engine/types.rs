
use std::{cell::RefCell,rc::Rc};


pub enum NodeId {
    BoxId(BoxId),
    ArrowId(Arrow),
}

pub struct BoxId {
    content:    String,
    right:      Option<Rc<RefCell<NodeId>>>,
    left:       Option<Rc<RefCell<NodeId>>>,
}

pub enum ArrowKind {
    Left,
    Right
}
pub struct Arrow {
    parent:  Option<Rc<RefCell<NodeId>>>,
    dimensions: (usize, usize),
    kind: Option<ArrowKind>,
}


