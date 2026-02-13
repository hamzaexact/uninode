use std::cmp::max;
use std::collections::HashMap;
use comfy_table::Color::White;
use crate::engine::engine;
use crate::engine::render::*;

impl engine::Engine {
    pub fn draw(&mut self) {
        for (_, node) in self.cells.iter() {
            engine::Engine::write_to_eninge_buffer(&mut self.buffer, node);
            self.max_x = max(self.max_x, node.get_position().col + node.get_length());
            self.max_y = max(self.max_y, node.get_position().row + node.get_dimentions().height);
        }
    }

    fn write_to_eninge_buffer(engine_buffer: &mut Vec<Vec<char>>, node: &RenderedNode)
    {
        match node {
            RenderedNode::Box(box_node) => {
                for (outer_index ,line) in box_node.content.lines().enumerate() {
                    let current_row = box_node.row.unwrap() + outer_index;
                    for (inner_index, ch) in line.chars().enumerate() {
                        let current_col = box_node.col.unwrap() + inner_index;
                        engine_buffer[current_row][current_col] = ch;
                    }
                }
            }
            RenderedNode::LinkedArrow(arrow) => {
                for (outer_index ,line) in arrow.arrow_string.lines().enumerate() {
                    let current_row = arrow.row.unwrap() + outer_index;
                    for (inner_index, ch) in line.chars().enumerate() {
                        let current_col = arrow.col.unwrap() + inner_index;
                        engine_buffer[current_row][current_col] = ch;
                    }
                }
            }
        }
    }
    // fn write_to_engine_string(engine_buffer: &mut Vec<Vec<char>>, string: &mut String, node: &RenderedNode) {
    // }
}

