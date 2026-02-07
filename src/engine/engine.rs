
use super::types::NodeId;
use super::render::{RenderedNode, Render};
use std::{collections::HashMap, rc::Rc, cell::RefCell};


pub const DEFAULT_ROW:usize = 0;
pub const DEFAULT_ARR_SIZE:usize = 5;
pub const DEFAULT_COL:usize = 0;

pub struct Engine {
    
    cells: HashMap<Rc<RefCell<NodeId>>, Rc<RefCell<RenderedNode>>>,
    buffer: Vec<Vec<char>>,
    nodes: Vec<RenderedNode>
}

impl Engine {
    pub fn init(width: usize, height: usize) -> Self {
        Self {
            cells: HashMap::new(),
            buffer: vec![vec![' '; width]; height],
            nodes: Vec::<RenderedNode>::new()
        }
    }

    pub fn spawn(&mut self, node: NodeId) -> RenderedNode {
        let mut rendered_node = node.render(None);
        match rendered_node {
            RenderedNode::Box(ref mut rendered_b) => {
                rendered_b.row = Some(DEFAULT_ROW);
                rendered_b.col = Some(DEFAULT_COL);
            },
            _ => unreachable!()
        }
        rendered_node
    }
}

