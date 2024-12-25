use crate::prelude::*;

mod spawn;
mod utils;

pub use bevy::time::Time;
pub use spawn::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy)
            .add_systems(Startup, construct_spawners);
    }
}
