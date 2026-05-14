use crate::prelude::*;

pub fn tower_fire(
    mut commands: Commands,
    time: Res<Time>,
    tower_query: Query<&mut TowerStats>,
    enemy_query: Query<(&Mobile, &mut Transform, Entity), With<Enemy>>,
) {
    for mut tower_stats in tower_query {
        tower_stats.timer.tick(time.delta());
        if tower_stats.timer.just_finished() == true {
            for (enemy, transform, entity) in enemy_query.iter() {}
            commands.spawn(
                (Bullet {
                    damage: 10.0,
                    target: Entity::PLACEHOLDER,
                }),
            );
        }
    }
}
