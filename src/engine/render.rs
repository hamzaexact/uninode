#![allow(warnings)]

use std::{cell::RefCell, rc::Rc};
use std::fmt::format;
use comfy_table::{presets::ASCII_FULL, ContentArrangement, Table};
use comfy_table::Color::White;
use crossterm::event::KeyCode::PageDown;
use super::{calculate_dimensions, types::{NodeId, ArrowId, ArrowKind, BoxId, Dimentions}, engine::DEFAULT_ARROW_LENTGH};

pub trait Render {
     fn render(&mut self, arr_parent: Option<&RenderedNode>) -> RenderedNode;
}


#[derive(Debug, Clone)]
pub enum RenderedNode {
    Box(Box),
    LinkedArrow(Arrow)
}


pub struct Position {
    pub col: usize,
    pub row:usize
}
impl RenderedNode {
    pub fn get_length(&self) -> usize {
        match &self {
            RenderedNode::LinkedArrow(arrow) => {
                arrow.length
            }
            RenderedNode::Box(box_node) => {
                box_node.content.len()
            }
        }
    }

    pub fn get_position(&self) -> Position {
        match self {
            RenderedNode::Box(box_node) => {
                Position {
                    row: box_node.row.unwrap(),
                    col: box_node.col.unwrap()
                }
            }
            RenderedNode::LinkedArrow(arrow) => {
                Position {
                    row: arrow.row.unwrap(),
                    col: arrow.col.unwrap()
                }
            }
        }
        }

    pub fn get_dimentions(&self) -> Dimentions {
        match self {
            RenderedNode::Box(box_node) => {
                box_node.dimentions.clone()
            }
            RenderedNode::LinkedArrow(arrow) => {
                Dimentions {
                    width: arrow.length,
                    height: 0
                }
            }
        }
    }
    }


#[derive(
    Debug,
    Clone
)]
pub struct Box {
    pub content: String,
    pub dimentions: Dimentions,
    pub row: Option<usize>,
    pub col: Option<usize>,
    pub left_arrow: Option<Arrow>,
    pub right_arrow: Option<Arrow>
}


#[derive(
    Debug,
    Clone
)]
pub struct Arrow {
    pub arrow_string: String,
    pub row: Option<usize>,
    pub kind: ArrowKind,
    pub col: Option<usize>,
    pub length: usize,
    // if true
    // [space][OPTIONtail][arrow][OPTIONhead][space]
    // if not
    // [tail][arrow][head]
}

impl Render for NodeId {
    fn render(&mut self, arr_parent: Option<&RenderedNode>) -> RenderedNode {
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
                    dimentions: area,
                    row: None,
                    col: None,
                    left_arrow: None,
                    right_arrow: None
                };
                return RenderedNode::Box(rendered_box);
            }
            NodeId::ArrowId(arrow) => {
                if arrow.length.is_none() {
                    arrow.length = Some(DEFAULT_ARROW_LENTGH)
                }
                let arrow_as_string = {
                    let arrow_len = arrow.length.unwrap();
                    let mut arrow_buffer = vec![""; arrow_len + 4];
                    let mut counter = 0;

                    // optional left offset
                    if arrow.with_offset {
                        arrow_buffer[counter] = " ";
                        counter += 1;
                    }

                    // left arrow head
                    if let ArrowKind::Left = arrow.kind {
                        if arrow.has_head {
                            arrow_buffer[counter] = "<";
                            counter += 1;
                        }
                    }

                    // arrow body
                    let arrow_slice = vec!["-"; arrow_len];
                    arrow_buffer[counter..counter + arrow_len]
                        .copy_from_slice(&arrow_slice);
                    counter += arrow_len;

                    // right arrow head
                    if let ArrowKind::Right = arrow.kind {
                        if arrow.has_head {
                            arrow_buffer[counter] = ">";
                            counter += 1;
                        }
                    }

                    // optional right offset
                    if arrow.with_offset {
                        arrow_buffer[counter] = " ";
                        counter += 1;
                    }

                    dbg!(&arrow_buffer);

                    arrow_buffer.concat()
                };
                // RenderedNode::LinkedArrow();
                let parent = arr_parent.unwrap();
                // let arrow_col =
                // let arrow_row =
                let arrow_len = arrow_as_string.len();
                // println!("{}",parent.get_length());
                let new_arrow = Arrow {
                    arrow_string: arrow_as_string,
                    col: Some(parent.get_dimentions().width  + parent.get_position().col),
                    row: Some(parent.get_position().row + parent.get_dimentions().height / 2),
                    kind: arrow.kind.clone(),
                    length: arrow_len
                };
                return RenderedNode::LinkedArrow(new_arrow)
            }
        }
    }
}





