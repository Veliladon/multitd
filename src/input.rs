use crate::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CameraMovement {
    // Movement

    // Abilities
    #[actionlike(Axis)]
    Zoom,
    #[actionlike(DualAxis)]
    Pan,
}

pub const CAMERA_MIN_ZOOM: f32 = 5.0;
pub const CAMERA_MAX_ZOOM: f32 = 25.;
pub const CAMERA_SPEED: f32 = 200.;

pub struct ProcessInputPlugin;

impl Plugin for ProcessInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraMovement>::default())
            .add_systems(PostStartup, camera_control_setup)
            .add_systems(Update, player_control)
            .add_systems(Update, zoom_camera)
            .add_systems(Update, move_camera);
    }
}

pub fn camera_control_setup(mut commands: Commands, mut camera_query: Query<Entity, With<Camera>>) {
    let input_map = InputMap::default()
        .with_axis(CameraMovement::Zoom, MouseScrollAxis::Y)
        .with_dual_axis(CameraMovement::Pan, KeyboardVirtualDPad::WASD);

    let camera = camera_query.single_mut();
    commands
        .entity(camera)
        .insert(InputManagerBundle::with_map(input_map));
}

pub fn zoom_camera(
    mut camera_query: Query<(&mut Transform, &ActionState<CameraMovement>), With<Camera>>,
) {
    let (mut camera_transform, action_state) = camera_query.single_mut();

    camera_transform.translation.y -= action_state.value(&CameraMovement::Zoom);
    if camera_transform.translation.y < CAMERA_MIN_ZOOM
        || camera_transform.translation.y > CAMERA_MAX_ZOOM
    {
        camera_transform.translation.y = camera_transform
            .translation
            .y
            .clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);
        return;
    }

    camera_transform.translation.z += action_state.value(&CameraMovement::Zoom);
}

pub fn move_camera(
    mut camera_query: Query<(&mut Transform, &ActionState<CameraMovement>), With<Camera>>,
    time: Res<Time>,
) {
    let (mut camera_transform, action_state) = camera_query.single_mut();
    let mut axis_pair = action_state.clamped_axis_pair(&CameraMovement::Pan);
    axis_pair = axis_pair * time.delta_seconds() * CAMERA_SPEED;
    camera_transform.translation.x -= axis_pair.x;
    camera_transform.translation.z += axis_pair.y;
}

pub fn player_control() {}
