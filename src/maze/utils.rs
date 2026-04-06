pub use crate::prelude::*;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Exit {
    Open,
    #[default]
    Closed,
    Start,
    Finish,
}

/* #[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cell {
    pub north: Exit,
    pub east: Exit,
    pub south: Exit,
    pub west: Exit,
} */

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cell {
    pub exits: [Exit; 4],
}

#[derive(Debug, Resource)]
pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub entry: i32,
    pub exit: i32,
    pub tiles: Vec<Cell>,
}

#[derive(Debug, Resource)]
pub struct MazeGraph {
    pub nodes: Vec<[Option<usize>; 4]>,
}

pub struct MazeRoute {
    pub nodes: Vec<usize>,
}
