use super::render::{Render, RenderedNode};
use super::types::NodeId;
use std::cmp::max;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub const DEFAULT_ROW: usize = 0;
pub const DEFAULT_ARROW_LENTGH: usize = 5;
pub const DEFAULT_COL: usize = 0;

#[derive(Debug)]
pub struct Engine {
    pub cells: HashMap<usize, RenderedNode>,
    pub buffer: Vec<Vec<char>>,
    // pub nodes: Vec<RenderedNode>, //
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
            // nodes: Vec::<RenderedNode>::new(),
            output: String::new(),
            id_counter: 0,
            max_x: 0,
            max_y: 0,
        }
    }
    pub fn spawn_at(&mut self, mut node: NodeId, row: Option<usize>, col: Option<usize>) -> Cell {
        let mut rendered_node = node.render(None, None, None, None);
        let (mut height, mut width) = (0, 0);
        if let RenderedNode::Box(ref mut rendered_b) = rendered_node {
            rendered_b.row = Some(row.unwrap_or(DEFAULT_ROW));
            rendered_b.col = Some(col.unwrap_or(DEFAULT_COL));
            height = rendered_b.dimentions.height;
            width = rendered_b.dimentions.width;
            self.cells.insert(self.id_counter, rendered_node);
        }
        self.id_counter += 1;
        Cell {
            id: self.id_counter - 1,
            height,
            length: width,
            row: row.unwrap(),
            col: col.unwrap(),
        }
    }
    pub fn spawn_arrow(
        &mut self,
        mut arrow_node: NodeId,
        row: Option<usize>,
        height: Option<usize>,
        col: Option<usize>,
        other_height: Option<usize>,
    ) -> Cell {
        let arrow_node = arrow_node.render(row, height, col, other_height);
        // dbg!(&arrow_node);;
        let mut cell: Option<Cell> = None;
        if let RenderedNode::LinkedArrow(ref arrow) = arrow_node {
            let c = Cell {
                id: self.id_counter,
                row: arrow.row.unwrap(),
                col: arrow.col.unwrap(),
                height: 0,
                length: arrow.length,
            };
            cell = Some(c);
        }
        self.cells.insert(self.id_counter, arrow_node);
        self.id_counter += 1;
        cell.unwrap()
    }

    pub fn spawn_at_left(
        &mut self,
        original_node: &Cell,
        current_node: NodeId,
        arrow_id: NodeId,
    ) -> Cell {
        let mut node2 = self.spawn_at(current_node, Some(DEFAULT_ROW), Some(DEFAULT_COL));
        let mut borrowed_node2 = self.cells.get_mut(&node2.id).unwrap();
        if let RenderedNode::Box(box_2) = borrowed_node2 {
            if let NodeId::ArrowId(ref ref_arrow) = arrow_id {
                (*box_2).col = Some(
                    original_node.col
                        + original_node.length
                        + ref_arrow.length.unwrap()
                        + if ref_arrow.has_head { 1 } else { 0 }
                        + if ref_arrow.with_offset { 2 } else { 0 },
                );
                (*box_2).row = Some(original_node.row);
                node2.row = original_node.row;
                node2.col = original_node.col
                    + original_node.length
                    + ref_arrow.length.unwrap()
                    + if ref_arrow.has_head { 1 } else { 0 }
                    + if ref_arrow.with_offset { 2 } else { 0 };
            }
        }
        let arrow_cell = self.spawn_arrow(
            arrow_id,
            Some(original_node.row),
            Some(original_node.height),
            Some(original_node.col + original_node.length),
            Some(node2.height),
        );
        // drop(borrowed_node2);
        let mut borrowed_original_node  = self.cells.get_mut(&original_node.id).unwrap();
        if let RenderedNode::Box(rendered_node) = borrowed_original_node {
            rendered_node.right_neighbor = Some(node2.id);

        }
        // drop(borrowed_original_node);
        let mut borrowed_node2 = self.cells.get_mut(&node2.id).unwrap();
        if let RenderedNode::Box(rendered_node) = borrowed_node2 {
            {
                rendered_node.left_neighbor = Some(original_node.id);
            }

        }
            node2
    }
    // pub fn left_spawn()
}

pub struct Cell {
    pub id: usize,
    pub height: usize,
    pub length: usize,
    pub row: usize,
    pub col: usize,
}

