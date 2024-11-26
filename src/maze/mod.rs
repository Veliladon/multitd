mod render;

pub use render::*;

pub const MAP_HEIGHT: usize = 6;
pub const MAP_WIDTH: usize = 6;
pub const CELL_HEIGHT: usize = 6;
pub const CELL_WIDTH: usize = 6;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Exit {
    Open,
    #[default]
    Closed,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Cell {
    north: Exit,
    east: Exit,
    south: Exit,
    west: Exit,
}

pub struct Maze {
    pub tiles: Vec<Cell>,
}

pub fn map_idx(x: usize, y: usize) -> usize {
    (y * MAP_WIDTH) + x
}

impl Maze {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }
}
