use crate::engine::types::Dimentions;

pub mod types;
pub mod render;
pub mod engine;
pub mod draw;

pub fn calculate_dimensions(s:&String) -> Dimentions {
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();
    let height = lines.len();
    let width = lines
        .iter()
        .map(|l| l.len())
        .max()
        .unwrap_or(0);
    Dimentions {
        width,
        height
    }
}
