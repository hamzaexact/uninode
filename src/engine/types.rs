

use std::{cell::RefCell,rc::Rc};
// use std::ffi::os_str::len;
use crate::engine::render::Arrow;

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



#[derive(Debug, Clone)]
pub enum ArrowKind {
    Left,
    Right
}
#[derive(Debug, Clone)]
pub struct ArrowId {
    pub length: Option<usize>,
    pub with_offset:bool,
    pub parent:  Option<Rc<RefCell<NodeId>>>,
    pub kind: ArrowKind,
    pub has_head: bool
}

#[derive(Debug, Clone)]
pub struct Dimentions {
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
    pub fn new(arrow_kind: ArrowKind, length: Option<usize>, with_offset:bool, has_head: bool) -> NodeId {
        NodeId::ArrowId(Self {
            length,
            with_offset,
            parent: None,
            kind: arrow_kind,
            has_head
        })
    }
}

