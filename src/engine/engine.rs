
pub struct Engine {
    graphtracker: Vec<u16> 
}
impl Engine {
    pub fn init() -> Self {
        Self {
            graphtracker: Vec::new()
        }
    }
}
