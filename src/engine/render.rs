#![allow(warnings)]

use std::{cell::RefCell, rc::Rc};
use std::cmp::min;
use std::fmt::format;
use comfy_table::{presets::ASCII_FULL, ContentArrangement, Table};
use comfy_table::Color::White;
use crossterm::event::KeyCode::PageDown;
use super::{calculate_dimensions, types::{NodeId, ArrowId, ArrowKind, BoxId, Dimentions}, engine::DEFAULT_ARROW_LENTGH};

pub trait Render {
    fn render(&mut self,row: Option<usize>, height: Option<usize>, other_height: Option<usize>, col: Option<usize>) -> RenderedNode;
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


    pub fn set_row(&mut self,row:usize) {
        match self {
            RenderedNode::Box(box_node) => {
                box_node.row = Some(row);
            }
            RenderedNode::LinkedArrow(arrow) => {
                arrow.row = Some(row);
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
    pub left_neighbor: Option<usize>,
    pub right_neighbor: Option<usize>
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
}

impl Render for NodeId {
    fn render(&mut self, row: Option<usize>,height: Option<usize>, col: Option<usize>,  other_height: Option<usize>) -> RenderedNode {
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
                    left_neighbor: None,
                    right_neighbor: None
                };
                return RenderedNode::Box(rendered_box);
            }
            NodeId::ArrowId(arrow) => {
                // let arrow_id_len = Default
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

                    // dbg!(&arrow_buffer);

                    arrow_buffer.concat()
                };
                let arrow_len = arrow_as_string.len();
                let new_arrow = Arrow {
                    arrow_string: arrow_as_string,
                    col,
                    row: Some(row.unwrap() + (min(height.unwrap(), other_height.unwrap()) / 2)),
                    kind: arrow.kind.clone(),
                    length: arrow_len
                };
                return RenderedNode::LinkedArrow(new_arrow)
            }
        }
    }
}



