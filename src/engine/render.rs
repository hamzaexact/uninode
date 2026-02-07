
#![allow(warnings)]


use std::{cell::RefCell, rc::Rc};
use comfy_table::presets::ASCII_FULL;
use comfy_table::{ContentArrangement, Table};
#[allow(unused_imports)]
use super::types::{NodeId, ArrowId, ArrowKind, BoxId, Area};
use super::calculate_dimensions;

pub trait Render {
     fn render(&self, arr_parent: Option<Rc<RefCell<NodeId>>>) -> RenderedNode;
}

pub enum RenderedNode {
    Box(Box),
    LinkedArrow(Arrow)
}

pub struct Box {
    pub content: String,
    pub area: Area,
    pub row:Option<usize>,
    pub col:Option<usize>,
    pub arrow: Option<ArrowId>
}


pub struct Arrow {
    pub parent:  Option<Rc<RefCell<NodeId>>>,
    pub kind: Option<ArrowKind>,
    pub row: Option<usize>,
    pub col: Option<usize>,
    pub length: usize,
}

impl Render for NodeId{
    fn render(&self, arr_parent: Option<Rc<RefCell<NodeId>>>) -> RenderedNode {
        match self {
            NodeId::BoxId(box_id) => {

                let mut cell = Table::new();
                cell.load_preset(ASCII_FULL).set_content_arrangement(ContentArrangement::Dynamic)
                    .add_row(vec![&box_id.content]);
                let mut cell = cell.to_string();
                let area = calculate_dimensions(&cell);
                let rendered_box = Box {
                    content: cell,
                    area,
                    row:None,
                    col:None,
                    arrow:None
                };
                return RenderedNode::Box(rendered_box);
            }
            NodeId::ArrowId(arrow) => {

            }
        }
        todo!()
    }
}



