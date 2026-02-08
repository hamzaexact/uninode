
use crate::engine::engine;
use crate::engine::render::*;

impl engine::Engine {
    pub fn draw(&mut self) {
        for node in self.nodes.iter() {
            engine::Engine::write_to_eninge_buffer(&mut self.buffer, node);
            
        }
    }

    fn write_to_eninge_buffer(engine_buffer: &mut Vec<Vec<char>>, node: &RenderedNode)
    {
        match node {
            RenderedNode::Box(box_node) => {
                if box_node.col.unwrap() + box_node.area.width > engine_buffer[0].len()
                {
                    println!("ENGINE BUFFER[0] LENGHT {},  {}", engine_buffer[0].len(), box_node.area.width + box_node.col.unwrap());
                    panic!("BUFFER  __OVERFLOW")
                }
                if box_node.area.height + box_node.row.unwrap() > engine_buffer.len() {
                    println!("ENGINE BUFFER LENGHT {},  {}", engine_buffer.len(), box_node.area.height + box_node.row.unwrap());
                    panic!("BUFFER  OVERFLOW")
                }
                for (outer_index ,line) in box_node.content.lines().enumerate() {
                    let current_row = box_node.row.unwrap() + outer_index;
                    for (inner_index, ch) in line.chars().enumerate() {
                        let current_col = box_node.col.unwrap() + inner_index;
                        engine_buffer[current_row][current_col] = ch;
                    }
                }
            }
            _ => todo!()
        }
    }
    // fn write_to_engine_string(engine_buffer: &mut Vec<Vec<char>>, string: &mut String, node: &RenderedNode) {
    // }
}
