// use assets::AssetLoadingPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraSetupPlugin;
use input::ProcessInputPlugin;

// mod assets;
mod camera;
mod input;

#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                    custom_layer: |_| None,
                }),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ProcessInputPlugin)
        // .add_plugins(AssetLoadingPlugin)
        .add_plugins(CameraSetupPlugin)
        .add_systems(Startup, scene_setup)
        .run();
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));
}
