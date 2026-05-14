use bevy::color::palettes::css::GREY;

pub use crate::prelude::*;

pub fn generate_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cylinder_handle = meshes.add(Cylinder::default());
    let cylinder_material = materials.add(Color::from(GREY));

    let tower_assets = TowerAssets {
        cylinder_handle,
        cylinder_material,
    };
    commands.insert_resource(tower_assets)
}

pub fn on_click_spawn_tower(
    _click: On<Pointer<Click>>,
    mut commands: Commands,
    tower_assets: Res<TowerAssets>,
) {
    commands
        .entity(_click.event_target())
        .with_children(|sub_parent| {
            sub_parent.spawn((
                Mesh3d(tower_assets.cylinder_handle.clone()),
                MeshMaterial3d(tower_assets.cylinder_material.clone()),
                Transform::from_xyz(0., 0.5, 0.),
                TowerStats {
                    damage: 2.0,
                    timer: Timer::from_seconds(0.8, TimerMode::Repeating),
                },
            ));
        });
}
