#![allow(warnings)]
mod engine;

use comfy_table::Color::White;
use engine::types::BoxId;
use engine::engine::Engine;
use crate::engine::engine::{DEFAULT_COL, DEFAULT_ROW};
use crate::engine::types::{ArrowId, ArrowKind};

fn main() {

    let mut engine = Engine::init(10 ,100);
    let mut root = BoxId::new("L\nEFT");
    let box_1 = engine.spawn_at(root, Some(DEFAULT_ROW), Some(DEFAULT_COL));
    let mut edge = ArrowId::default();
    let box_2 = engine.spawn_at_left(&box_1,BoxId::new("BOX"),edge);
    let box_3 = engine.spawn_at_left(&box_2, BoxId::new("B\nO\nX"),ArrowId::default());
    let box_4 = engine.spawn_at_left(&box_3, BoxId::new("USER\n2\nE"),ArrowId::default());
    engine.draw();
    let output = engine.buffer.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);
}