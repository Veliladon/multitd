use crate::prelude::*;
use bevy::dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings};

pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InfiniteGridPlugin)
            .add_systems(Startup, infinite_grid)
            .add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
    ));
}

fn infinite_grid(mut commands: Commands) {
    commands.spawn((
        // You need to spawn an entity with this component
        InfiniteGrid,
        // Optional component you can use to configure the grid
        InfiniteGridSettings::default(),
    ));
}
