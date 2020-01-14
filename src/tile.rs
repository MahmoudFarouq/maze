#[derive(PartialEq, Copy, Clone)]
pub enum TileState {
    UnDiscovered,
    Discovered,
    Visited
}