
use std::{cell::RefCell,rc::Rc};


pub enum NodeId {
    BoxId(BoxId),
    ArrowId(ArrowId),
}

pub struct BoxId {
    pub content:    String,
    pub right:      Option<Rc<RefCell<NodeId>>>,
    pub left:       Option<Rc<RefCell<NodeId>>>,
}

pub enum ArrowKind {
    Left,
    Right
}
pub struct ArrowId {
    pub parent:  Option<Rc<RefCell<NodeId>>>,
    pub kind: Option<ArrowKind>,
}


pub struct Area {
    pub width: usize,
    pub height: usize
}


