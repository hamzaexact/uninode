//

use std::{cell::RefCell,rc::Rc};



pub enum Node {
    Box,
}

pub struct Box<'a> {

    content:    &'a str,
    dimention:  BoxDimension,
    right:      Option<Rc<RefCell<Node>>>,
    left:       Option<Rc<RefCell<Node>>>,
    up:         Option<Rc<RefCell<Node>>>,
    down:       Option<Rc<RefCell<Node>>>,
    is_root:    bool

}

pub struct BoxDimension {
    x: i32,
    y: i32
}

