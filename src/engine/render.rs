#![allow(warnings)]

use std::{cell::RefCell, rc::Rc};
use std::fmt::format;
use comfy_table::{presets::ASCII_FULL, ContentArrangement, Table};
use comfy_table::Color::White;
use super::{calculate_dimensions, types::{NodeId, ArrowId, ArrowKind, BoxId, Area}, engine::DEFAULT_ARROW_LENTGH};

pub trait Render {
     fn render(&mut self, arr_parent: Option<Rc<RefCell<NodeId>>>) -> RenderedNode;
}


#[derive(Debug, Hash)]
pub enum RenderedNode {
    Box(Box),
    LinkedArrow(Arrow)
}

#[derive(Debug, Hash)]
pub struct Box {
    pub content: String,
    pub area: Area,
    pub row:Option<usize>,
    pub col:Option<usize>,
    pub arrow: Option<ArrowId>
}


#[derive(Debug, Hash)]
pub struct Arrow {
    pub parent:  Option<Rc<RefCell<NodeId>>>,
    pub kind: Option<ArrowKind>,
    pub row: Option<usize>,
    pub col: Option<usize>,
    pub length: usize,
    // if true
    // [space][tail][arrow][head][space]
    // if not
    // [tail][arrow][head]
    pub with_offset: bool
}
impl Render for NodeId {
    fn render(&mut self, arr_parent: Option<Rc<RefCell<NodeId>>>) -> RenderedNode {
        match self {
            NodeId::BoxId(box_id) => {
                let k = 1;
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
                if arrow.lenght.is_none() {
                    arrow.lenght = Some(DEFAULT_ARROW_LENTGH)
                }
                let arrow_as_string = {
                    let arrow_len = arrow.lenght.unwrap();
                    let mut arrow_buffer = vec![""; arrow_len+3];
                    arrow_buffer[0] = {
                        if arrow.with_offset {""} else {" "}
                    };
                    arrow_buffer[1] = {
                        if let Some(ref direction) = arrow.kind {
                            if let direction = ArrowKind::Left { "<"} else { "" }
                        } else { "" }};
                    let arrow_slice = vec!["-"; arrow_len];
                    &arrow_buffer[2..].copy_from_slice(&arrow_slice);
                    let mut counter = arrow_len+1;
                    arrow_buffer[counter] = {
                        if let Some(ref direction) = arrow.kind {
                            if let direction = ArrowKind::Right {
                                counter+=1;
                                ">"
                            } else {""}
                        } else {""}
                    };
                    arrow_buffer[counter] = {
                        if arrow_buffer[0] == " " {" "} else {""}
                    };

                    arrow_buffer.iter().map(|ch| *ch).collect::<String>()
                };
            }
        }
        todo!()
    }
}



