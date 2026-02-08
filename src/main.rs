#![allow(warnings)]
mod engine;
use engine::types::BoxId;
use engine::engine::Engine;
fn main() {
    let mut engine = Engine::init(10, 3);
    let root_node = BoxId::new("HAMZA");
    engine.spawn(root_node);
    for l in engine.buffer.iter() {
        for i in l.iter() {
            print!("{i}");
        }
        println!("");
    }

}
