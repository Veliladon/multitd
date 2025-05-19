use crate::prelude::*;

use leafwing_input_manager::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum InputState {
    Standard,
    #[default]
    Debug,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CameraMovement {
    // Abilities
    #[actionlike(Axis)]
    Zoom,
    #[actionlike(DualAxis)]
    Pan,
    SwitchInputType,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum BuilderAction {
    // Abilities
    NextMesh,
    PrevMesh,
}

pub const CAMERA_MIN_ZOOM: f32 = 5.0;
pub const CAMERA_MAX_ZOOM: f32 = 25.;
pub const CAMERA_SPEED: f32 = 50.;

pub struct ProcessInputPlugin;

impl Plugin for ProcessInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraMovement>::default())
            .add_systems(PostStartup, camera_control_setup)
            .init_state::<InputState>()
            .register_type::<InputState>()
            /* .add_plugins(EguiPlugin {
                enable_multipass_for_primary_context: true,
            })
            .add_plugins(StateInspectorPlugin::<InputState>::default()) */
            .add_systems(Update, player_control)
            .add_systems(Update, switch_input_types)
            .add_systems(Update, zoom_camera)
            .add_systems(Update, move_camera)
            .add_systems(Update, builder_control);
    }
}

pub fn camera_control_setup(mut commands: Commands, mut camera_query: Query<Entity, With<Camera>>) {
    let input_map = InputMap::default()
        .with(CameraMovement::SwitchInputType, KeyCode::Backquote)
        .with_axis(CameraMovement::Zoom, MouseScrollAxis::Y)
        .with_dual_axis(CameraMovement::Pan, VirtualDPad::wasd());

    let builder_input_map = InputMap::default()
        .with(BuilderAction::PrevMesh, KeyCode::KeyQ)
        .with(BuilderAction::NextMesh, KeyCode::KeyE);

    let camera = camera_query.single_mut().unwrap();
    commands
        .entity(camera)
        .insert(input_map)
        .insert(builder_input_map);
}

pub fn zoom_camera(
    mut camera_query: Query<(&mut Transform, &ActionState<CameraMovement>), With<Camera>>,
) {
    let (mut camera_transform, action_state) = camera_query.single_mut().unwrap();

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

    camera_transform.translation.z -= action_state.value(&CameraMovement::Zoom);
}

pub fn switch_input_types(
    input_query: Query<&ActionState<CameraMovement>>,
    current_state: Res<State<InputState>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    let action_state = input_query.single().unwrap();
    if action_state.just_pressed(&CameraMovement::SwitchInputType) {
        next_state.set(match current_state.get() {
            InputState::Standard => InputState::Debug,
            InputState::Debug => InputState::Standard,
        });
    }
}

pub fn move_camera(
    mut camera_query: Query<(&mut Transform, &ActionState<CameraMovement>), With<Camera>>,
    time: Res<Time>,
) {
    let (mut camera_transform, action_state) = camera_query.single_mut().unwrap();
    let mut axis_pair = action_state.clamped_axis_pair(&CameraMovement::Pan);
    axis_pair = axis_pair * time.delta_secs() * CAMERA_SPEED;
    camera_transform.translation.x += axis_pair.x;
    camera_transform.translation.z -= axis_pair.y;
}

pub fn builder_control(mut camera_query: Query<&ActionState<BuilderAction>, With<Camera>>) {
    let action_state = camera_query.single_mut().unwrap();
    if action_state.just_pressed(&BuilderAction::NextMesh) {}

    if action_state.just_pressed(&BuilderAction::PrevMesh) {}
}

pub fn player_control() {}
