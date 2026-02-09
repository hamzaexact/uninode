
use super::types::NodeId;
use super::render::{RenderedNode, Render};
use std::{collections::HashMap, rc::Rc, cell::RefCell};


pub const DEFAULT_ROW:usize = 0;
pub const DEFAULT_ARROW_LENTGH:usize = 5;
pub const DEFAULT_COL:usize = 0;

#[derive(Debug)]
pub struct Engine {
    
    pub cells: HashMap<Rc<RefCell<NodeId>>, Rc<RefCell<RenderedNode>>>,
    pub buffer: Vec<Vec<char>>,
    pub nodes: Vec<RenderedNode>,
    pub output: String,
    pub max_x: usize,
    pub max_y: usize,

}

impl Engine {

    pub fn init(height: usize, width: usize) -> Self {
        Self {
            cells: HashMap::new(),
            buffer: vec![vec![' '; width]; height],
            nodes: Vec::<RenderedNode>::new(),
            output: String::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn spawn(&mut self, mut node: NodeId) {
        let mut rendered_node = node.render(None);
        match rendered_node {
            RenderedNode::Box(ref mut rendered_b) => {
                rendered_b.row = Some(DEFAULT_ROW);
                rendered_b.col = Some(DEFAULT_COL);
                self.cells.insert(Rc::new(RefCell::new(&node)),Rc::new(RefCell::new(&rendered_b)));
                self.nodes.push(rendered_node);
            },
            _ => todo!()
        }
    }

}

