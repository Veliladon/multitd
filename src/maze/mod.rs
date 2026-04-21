mod abmaze;

mod setup;

pub mod utils;

pub use rand::prelude::*;

pub use setup::*;

pub use utils::*;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_maze);
        app.add_systems(Startup, generate_maze_entities.after(generate_maze))
            //.add_systems(Startup, generate_tree.after(generate_maze))
            .add_systems(Startup, scene_setup.after(generate_maze_entities));
    }
}
