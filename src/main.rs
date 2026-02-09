#![allow(warnings)]
mod engine;
use engine::types::BoxId;
use engine::engine::Engine;
fn main() {

    let mut engine = Engine::init(5, 12);
    let mut new_node = BoxId::new("HAMZA");
    engine.spawn(new_node);
    engine.draw();
    let output = engine.buffer.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);

}
