
use std::{cell::RefCell,rc::Rc};


#[derive(Debug)]
pub enum NodeId {
    BoxId(BoxId),
    ArrowId(ArrowId),
}



#[derive(Debug)]
pub struct BoxId {
    pub content:    String,
    pub right:      Option<Rc<RefCell<NodeId>>>,
    pub left:       Option<Rc<RefCell<NodeId>>>,
}
impl BoxId {
    pub fn new(content: &str) -> NodeId {
        let box_node = Self {
            content: content.to_string(),
            left: None,
            right: None
        };
        NodeId::BoxId(box_node)
    }
}
#[derive(Debug)]
pub enum ArrowKind {
    Left,
    Right
}
#[derive(Debug)]
pub struct ArrowId {
    pub parent:  Option<Rc<RefCell<NodeId>>>,
    pub kind: Option<ArrowKind>,
}

#[derive(Debug)]
pub struct Area {
    pub width: usize,
    pub height: usize
}


