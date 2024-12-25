use std::time::Duration;

use crate::prelude::*;

use super::utils::EnemySpawner;

pub fn construct_spawners(mut commands: Commands) {
    commands.spawn(EnemySpawner {
        timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
    });
}

pub fn spawn_enemy(time: Res<Time>, mut spawner_query: Query<&mut EnemySpawner>) {
    for mut spawner in &mut spawner_query {
        spawner.timer.tick(time.delta());
    }
}
