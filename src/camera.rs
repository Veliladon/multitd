use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;

pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .insert_resource(Msaa::Off);
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),

            ..default()
        })
        .insert(ScreenSpaceAmbientOcclusionBundle::default());
}
