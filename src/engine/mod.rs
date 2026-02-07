use crate::engine::types::Area;

pub mod types;
pub mod render;
pub mod engine;

pub fn calculate_dimensions(s:&String) -> Area {
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();
    let height = lines.len();
    let width = lines
        .iter()
        .map(|l| l.len())
        .max()
        .unwrap_or(0);
    Area {
        width,
        height
    }
}
