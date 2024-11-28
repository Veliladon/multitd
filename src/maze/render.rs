use crate::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy_mod_picking::prelude::*;
use std::f32::consts::{PI, TAU};

#[derive(Component)]
struct Ground;

#[derive(Component, Deref, DerefMut)]
struct GridPos(IVec2);

/* #[derive(Component)]
struct HoverSelection; */

pub struct MazePlugin;

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

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_maze)
            .add_systems(Startup, scene_setup);
    }
}

fn generate_maze(mut commands: Commands) {
    let rng = thread_rng();
    let maze = Maze::new(6, 6, rng);
    commands.insert_resource(maze);
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    game_assets: Res<GameAssets>,
    maze: Res<Maze>,
) {
    // Ambient Light

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    /* commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 10000.00,
    }); */

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
                        GridPos {
                            0: IVec2::new(x as i32, z as i32),
                        },
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
