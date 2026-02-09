
use std::{cell::RefCell,rc::Rc};
use crate::engine::render::Arrow;

#[derive(Debug, Hash)]
pub enum NodeId {
    BoxId(BoxId),
    ArrowId(ArrowId),
}



#[derive(Debug, Hash)]
pub struct BoxId {
    pub content:    String,
    pub right:      Option<Rc<RefCell<NodeId>>>,
    pub left:       Option<Rc<RefCell<NodeId>>>,
}



#[derive(Debug, Hash)]
pub enum ArrowKind {
    Left,
    Right
}
#[derive(Debug, Hash)]
pub struct ArrowId {
    pub lenght: Option<usize>,
    pub parent:  Option<Rc<RefCell<NodeId>>>,
    pub with_offset:bool,
    pub kind: Option<ArrowKind>,
}

#[derive(Debug,  Hash)]
pub struct Area {
    pub width: usize,
    pub height: usize
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



impl ArrowId {
    fn new(arrow_kind: Option<ArrowKind>, lenght: Option<usize>, with_offset:bool) -> Self {
        Self {
            lenght,
            with_offset,
            parent: None,
            kind: arrow_kind
        }
    }
}

