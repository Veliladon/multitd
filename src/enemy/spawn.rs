use bevy::prelude::*;

pub struct EnemySpawner {}

pub fn spawn_enemy(mut commands: Commands, time: Res<Time>) {
    commands.spawn(());
}
