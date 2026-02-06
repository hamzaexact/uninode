#[allow(unused_imports)]
use super::types::{NodeId, Arrow, ArrowKind, BoxId};

pub trait Render {
     fn render(&self) -> RenderedNode;
}

pub enum RenderedNode {
    Box(Box),
    LinkedArrow(Arrow)
}
pub struct Box {
    content: String,
    width: usize,
    height: usize,
    row:Option<usize>,
    col:Option<usize>,
    arrow: Option<Arrow>
}

impl Render for Box {
    fn
}




