
use std::{cell::RefCell,rc::Rc};



pub enum Node {
    Box,
}

pub struct Box<'a> {

    content:    &'a str, // for the moment i'll stick with static strings
    dimention:  BoxDimension,
    right:      Option<Rc<RefCell<Node>>>,
    left:       Option<Rc<RefCell<Node>>>,
    up:         Option<Rc<RefCell<Node>>>,
    down:       Option<Rc<RefCell<Node>>>,
    // I may need to comment this out later
    // is_root:    bool

}

pub struct BoxDimension {
    height: u16,
    width: u16,
    start_x: u16,
    end_x: u16,
    start_y: u16,
    end_y: u16
}

