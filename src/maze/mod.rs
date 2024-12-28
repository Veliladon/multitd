mod abmaze;
mod render;
mod tree;
mod utils;

pub use abmaze::*;
pub use rand::prelude::*;
pub use render::*;
pub use tree::*;
pub use utils::*;

pub const MAP_HEIGHT: usize = 6;
pub const MAP_WIDTH: usize = 6;
pub const CELL_HEIGHT: usize = 6;
pub const CELL_WIDTH: usize = 6;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_maze)
            .add_systems(Startup, generate_tree.after(generate_maze))
            .add_systems(Startup, scene_setup.after(generate_maze));
    }
}
