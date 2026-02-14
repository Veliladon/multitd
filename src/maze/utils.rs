pub use crate::prelude::*;
pub use rand::prelude::*;

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

pub trait MazeBuilder {
    fn new(width: i32, height: i32, rng: ThreadRng) -> Maze;
}

impl Maze {
    pub fn new(width: i32, height: i32, rng: ThreadRng) -> Maze {
        ABMaze::new(width, height, rng)
    }

    pub fn idx(&self, pos: IVec2) -> usize {
        ((pos.y * (self.width as i32)) + pos.x) as usize
    }

    pub fn in_bounds(&self, pos: IVec2) -> bool {
        (pos.x >= 0)
            && (pos.x < (self.width as i32))
            && (pos.y >= 0)
            && (pos.y < (self.height as i32))
    }
}
