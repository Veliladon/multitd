use super::*;
use bevy::color::palettes::css::*;
use bevy_mod_picking::prelude::*;
use std::f32::consts::TAU;

#[derive(Component)]
struct Ground;

/* #[derive(Component)]
struct HoverSelection; */

pub struct MapPlugin;

pub const MAP_HEIGHT: usize = 6;
pub const MAP_WIDTH: usize = 6;
pub const CELL_HEIGHT: usize = 6;
pub const CELL_WIDTH: usize = 6;

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.5, -0.3, 0.9, 0.8), 0.5), // hovered is blue
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.4, -0.4, 0.8, 0.8), 0.5), // pressed is a different blue
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.4, 0.8, -0.4, 0.0), 0.5), // selected is green
        ..matl.to_owned()
    })),
};

/* pub enum MapPieces {
    FourWay,
    ThreeWay,
    TwoWay,
    EndCap,
} */

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scene_setup);
    }
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    game_assets: Res<GameAssets>,
) {
    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 10000.00,
    });

    // Ground Plane

    commands
        .spawn_empty()
        .insert(SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            for z in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    let z_pos = (z * CELL_HEIGHT) as f32;
                    let x_pos = (x * CELL_WIDTH) as f32;
                    parent.spawn((
                        PbrBundle {
                            mesh: meshes.add(Plane3d::default().mesh().size(6., 6.)),
                            material: game_assets.ground_material_handle.clone(),
                            transform: Transform::from_xyz(x_pos, 0.5, z_pos),
                            ..default()
                        },
                        Ground,
                        PickableBundle::default(),
                        HIGHLIGHT_TINT,
                    ));
                }
            }
        });

    /* commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(36., 36.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.0)),
            ..default()
        },
        Ground,
    )); */

    // 4-Way Junction
    commands.spawn(SceneBundle {
        scene: game_assets.fourway_handle.clone(),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
    });

    // 3-Way Junction
    commands.spawn(SceneBundle {
        scene: game_assets.threeway_handle.clone(),
        transform: Transform::from_xyz(6., 0.5, 0.),
        ..default()
    });

    // 2-Way Junction

    commands.spawn(SceneBundle {
        scene: game_assets.twoway_handle.clone(),
        transform: Transform::from_xyz(12., 0.5, 0.),
        ..default()
    });

    // End Cap
    commands.spawn(SceneBundle {
        scene: game_assets.endcap_handle.clone(),
        transform: Transform::from_xyz(0., 0.5, 6.),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: game_assets.endcap_handle.clone(),
        transform: Transform::from_xyz(6., 0.5, 6.)
            .with_rotation(Quat::from_rotation_y(TAU / 4. * 3.)),
        ..default()
    });
}

/* fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    // Ambient Light
    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 10000.00,
    });

    // Road Plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(36., 36.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.0)),
            ..default()
        },
        Ground,
    ));

    // 4-Way Junction
    commands.spawn(SceneBundle {
        scene: game_assets.fourway_handle.clone(),
        transform: Transform::from_xyz(3., 0., 0.),
        ..default()
    });

    // 3-Way Junction
    commands.spawn(SceneBundle {
        scene: game_assets.threeway_handle.clone(),
        transform: Transform::from_xyz(9., 0., 0.),
        ..default()
    });

    // 2-Way Junction

    commands.spawn(SceneBundle {
        scene: game_assets.twoway_handle.clone(),
        transform: Transform::from_xyz(15., 0., 0.),
        ..default()
    });

    // End Cap
    commands.spawn(SceneBundle {
        scene: game_assets.endcap_handle.clone(),
        transform: Transform::from_xyz(3., 0., 6.),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: game_assets.endcap_handle.clone(),
        transform: Transform::from_xyz(9., 0., 6.).with_rotation(Quat::from_rotation_y(TAU / 4.)),
        ..default()
    });
}
 */
