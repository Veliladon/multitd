use bevy::prelude::*;

mod spawn;

use spawn::*;

#[derive(Debug, Resource)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy);
    }
}
