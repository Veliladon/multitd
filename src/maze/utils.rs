pub use crate::prelude::*;
pub use crate::utils::Direction;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Exit {
    Open,
    #[default]
    Closed,
    Start,
    Finish,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cell {
    pub exits: [Exit; 4],
}

#[derive(Debug, Resource)]
pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub entry_index: usize,
    pub exit_index: usize,
    pub tiles: Vec<Cell>,
}

#[derive(Debug, Resource)]
pub struct MazeGraph {
    pub nodes: Vec<[Option<usize>; 4]>,
}

#[derive(Debug, Resource)]
pub struct MazeRoute {
    pub nodes: Vec<usize>,
    pub entry_direction: Direction,
    pub exit_direction: Direction,
}

#[derive(Resource, Deref, DerefMut)]
pub struct MazeEntityMap(pub Vec<Entity>);
