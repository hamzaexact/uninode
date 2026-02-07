#![allow(warnings)]
use comfy_table::{presets::{self, ASCII_FULL}, ContentArrangement, Table};
mod engine;

use engine::engine::Engine;
use crossterm::{
    cursor,
    execute,
    style::Print,
    terminal::{self, ClearType},
};
use std::{cmp::max, io::stdout};

fn main() {
    let mut stdout = stdout();
    let mut cell = Table::new();
    cell.load_preset(ASCII_FULL).set_content_arrangement(ContentArrangement::Dynamic)
    .add_row(vec!["HAMZA"]);

    let lines: Vec<String> = cell.lines().map(|l| l.to_string()).collect();
    let height = lines.len();
    let width = lines
        .iter()
        .map(|l| l.len())
        .max()
        .unwrap_or(0);

}
