use crate::prelude::*;

mod movement;
mod spawn;
mod utils;

pub use bevy::time::Time;
pub use movement::*;
pub use spawn::*;
pub use utils::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, spawn_enemy)
            .add_systems(Startup, create_mesh)
            .add_systems(Startup, load_enemy_assets)
            .add_systems(Startup, construct_spawners);
    }
}
