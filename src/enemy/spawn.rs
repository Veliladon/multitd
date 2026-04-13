use std::time::Duration;

use bevy::color::palettes::tailwind::CYAN_300;

use crate::{enemy::utils::*, maze::utils::MazeRoute, prelude::*};

use super::utils::EnemySpawner;

pub fn construct_spawners(mut commands: Commands) {
    commands.spawn(EnemySpawner {
        timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
        counter: 10,
    });
}

pub fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh_handle = meshes.add(Cuboid::new(0.7, 0.7, 0.7));
    let cube_material_handle = materials.add(Color::from(CYAN_300));
    let enemy_mesh_handle = EnemyMeshHandle {
        cube_mesh_handle,
        cube_material_handle,
    };
    commands.insert_resource(enemy_mesh_handle);
}

pub fn load_enemy_assets() {}

pub fn spawn_enemy(
    time: Res<Time>,
    mut spawner_query: Query<&mut EnemySpawner>,
    enemy_mesh_handles: Res<EnemyMeshHandle>,
    mut commands: Commands,
    enemy_path: Res<MazeRoute>,
    entity_map: Res<MazeEntityMap>,
) {
    let start_grid_index = enemy_path.nodes[0];
    let first_destination_index = enemy_path.nodes[1];
    for mut spawner in &mut spawner_query {
        spawner.timer.tick(time.delta());
        if spawner.timer.just_finished() && spawner.counter > 0 {
            commands
                .entity(entity_map[start_grid_index])
                .with_children(|sub_parent| {
                    sub_parent.spawn_empty().insert((
                        Transform::from_xyz(0., 0.35, -3.),
                        Visibility::default(),
                        Mesh3d(enemy_mesh_handles.cube_mesh_handle.clone()),
                        MeshMaterial3d(enemy_mesh_handles.cube_material_handle.clone()),
                        Mobile {
                            speed: 2.,
                            destination: first_destination_index,
                            direction: crate::utils::DIRECTIONS[0],
                        },
                    ));
                });
        }
    }
}
