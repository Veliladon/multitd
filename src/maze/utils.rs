pub use crate::prelude::*;
pub use crate::utils::Direction;

pub const MAP_HEIGHT: usize = 6;
pub const MAP_WIDTH: usize = 6;
pub const CELL_HEIGHT: f32 = 6.;
pub const CELL_WIDTH: f32 = 6.;

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

#[derive(Resource)]
pub struct TileMaterials {
    pub ground_mesh_handle: Handle<Mesh>,
    pub ground_material_handle: Handle<StandardMaterial>,
    pub cube_mesh_handle: Handle<Mesh>,
    pub wall_cube_material_handle: Handle<StandardMaterial>,
    pub place_cube_material_handle: Handle<StandardMaterial>,
    pub place_cube_hover_handle: Handle<StandardMaterial>,
    pub place_cube_pressed_handle: Handle<StandardMaterial>,
    pub short_wall_mesh_handle: Handle<Mesh>,
}
