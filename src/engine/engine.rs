
use super::types::NodeId;
use super::render::{RenderedNode, Render};
use std::{collections::HashMap, rc::Rc, cell::RefCell};
use std::cmp::max;

pub const DEFAULT_ROW:usize = 0;
pub const DEFAULT_ARROW_LENTGH:usize = 5;
pub const DEFAULT_COL:usize = 0;

#[derive(Debug)]
pub struct Engine {
    
    pub cells: HashMap<usize, Rc<RefCell<RenderedNode>>>,
    pub buffer: Vec<Vec<char>>,
    pub nodes: Vec<RenderedNode>,
    pub output: String,
    pub id_counter: usize,
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
            id_counter:0,
            max_x: 0,
            max_y: 0,
        }
    }
    pub fn spawn_at(&mut self, mut node: NodeId, row: Option<usize>, col: Option<usize>) -> usize {
        let mut rendered_node = node.render(None);
        match rendered_node {
            RenderedNode::Box(ref mut rendered_b) => {
                rendered_b.row = Some({
                    if row.is_some() {row.unwrap()} else{DEFAULT_ROW}}
                );
                rendered_b.col = Some(
                    if col.is_some() {col.unwrap()} else {DEFAULT_COL}
                );
                self.cells.insert(self.id_counter, Rc::new(RefCell::new(rendered_node.clone())));
                self.nodes.push(rendered_node);
            },
            _ => todo!()
        }
        self.id_counter +=1;
        self.id_counter -1
    }
    pub fn spawn_arrow(&mut self, mut arrow_node:NodeId, parent_id: usize) {
        let from = &*(self.cells.get(&parent_id).unwrap()).borrow();
        let arrow_node =  arrow_node.render(Some(from));
        dbg!(&arrow_node);
        // self.cells.insert(self.id_counter, Rc::new(RefCell::new(rendered_node.clone())));
        self.nodes.push(arrow_node);
        /*match from {
            RenderedNode::Box(box_node) => {
            }
            _ => unreachable!()
        }*/
        // match from
    }

    // pub fn spawn

    pub fn update_max_x_y(&mut self, target_node: &RenderedNode) {
        self.max_x = max(self.max_x, target_node.get_position().col + target_node.get_length());
        self.max_y = max(self.max_y, target_node.get_position().row + target_node.get_dimentions().height);
    }
}

