mod abmaze;
mod render;

use std::collections::BTreeMap;

pub use crate::prelude::*;
pub use abmaze::*;
pub use rand::prelude::*;
pub use render::*;

pub const MAP_HEIGHT: usize = 6;
pub const MAP_WIDTH: usize = 6;
pub const CELL_HEIGHT: usize = 6;
pub const CELL_WIDTH: usize = 6;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Exit {
    Open,
    #[default]
    Closed,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cell {
    north: Exit,
    east: Exit,
    south: Exit,
    west: Exit,
}
#[derive(Debug, Resource)]
pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub entry: i32,
    pub exit: i32,
    pub tiles: Vec<Cell>,
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_maze)
            .add_systems(Startup, scene_setup.after(generate_maze));
    }
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

    pub fn generate_tree(&self) -> BTreeMap<IVec2, IVec2> {
        BTreeMap::new()
    }
}
