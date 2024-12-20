use std::f32::consts::TAU;

use bevy::color::palettes::tailwind::{CYAN_300, YELLOW_300};
//use bevy::picking::pointer::PointerInteraction;
use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component)]
struct Ground;

#[derive(Component, Deref, DerefMut)]
struct GridPos(IVec2);

pub const WALL_CUBE_COMMON: Transform = Transform::from_xyz(1.5, 0.5, 1.5);
pub const PLACE_CUBE_COMMON: Transform = Transform::from_xyz(2.5, 0.5, 2.5);
pub const WALL_CUBE_OPEN_LEFT: Transform = Transform::from_xyz(-1.5, 0.5, 2.5);
pub const WALL_CUBE_OPEN_RIGHT: Transform = Transform::from_xyz(1.5, 0.5, 2.5);
pub const WALL_LONG_CLOSED: Transform = Transform::from_xyz(0.0, 0.5, 2.5);
pub const PLACE_SHORT_CLOSED: Transform = Transform::from_xyz(0.0, 0.5, 1.5);

// pub const DIRECTION: [Quat; 4] = [Quat.0:(1, 0, 0, 0)];

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_maze)
            .add_systems(Startup, scene_setup.after(generate_maze));
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze: Res<Maze>,
) {
    let ground_mesh_handle = meshes.add(Plane3d::default().mesh().size(6., 6.));
    let ground_material_handle = materials.add(Color::srgb(0.3, 0.5, 0.0));
    let cube_mesh_handle = meshes.add(Cuboid::new(1., 1., 1.));
    let wall_cube_material_handle = materials.add(Color::srgb(0.75, 0.75, 0.75));
    let place_cube_material_handle = materials.add(Color::srgb(0.625, 0.32, 0.175));
    let place_cube_hover_handle = materials.add(Color::from(CYAN_300));
    let place_cube_pressed_handle = materials.add(Color::from(YELLOW_300));
    let long_wall_mesh_handle = meshes.add(Cuboid::new(4., 1., 1.));
    let short_wall_mesh_handle = meshes.add(Cuboid::new(2., 1., 1.));

    let east_quat = Quat::from_axis_angle(Vec3::Y, TAU / 4.0);
    let south_quat = Quat::from_axis_angle(Vec3::Y, TAU / 2.0);
    let west_quat = Quat::from_axis_angle(Vec3::Y, (TAU / 4.0) * 3.);

    let north_wall_cube_common_transform = WALL_CUBE_COMMON.clone();
    let mut east_wall_cube_common_transform = WALL_CUBE_COMMON.clone();
    east_wall_cube_common_transform.rotate_around(Vec3::ZERO, east_quat);
    let mut south_wall_cube_common_transform = WALL_CUBE_COMMON.clone();
    south_wall_cube_common_transform.rotate_around(Vec3::ZERO, south_quat);
    let mut west_wall_cube_common_transform = WALL_CUBE_COMMON.clone();
    west_wall_cube_common_transform.rotate_around(Vec3::ZERO, west_quat);

    let north_place_cube_common_transform = PLACE_CUBE_COMMON.clone();
    let mut east_place_cube_common_transform = PLACE_CUBE_COMMON.clone();
    east_place_cube_common_transform.rotate_around(Vec3::ZERO, east_quat);
    let mut south_place_cube_common_transform = PLACE_CUBE_COMMON.clone();
    south_place_cube_common_transform.rotate_around(Vec3::ZERO, south_quat);
    let mut west_place_cube_common_transform = PLACE_CUBE_COMMON.clone();
    west_place_cube_common_transform.rotate_around(Vec3::ZERO, west_quat);

    let north_wall_cube_open_left_transform = WALL_CUBE_OPEN_LEFT.clone();
    let mut east_wall_cube_open_left_transform = WALL_CUBE_OPEN_LEFT.clone();
    east_wall_cube_open_left_transform.rotate_around(Vec3::ZERO, east_quat);
    let mut south_wall_cube_open_left_transform = WALL_CUBE_OPEN_LEFT.clone();
    south_wall_cube_open_left_transform.rotate_around(Vec3::ZERO, south_quat);
    let mut west_wall_cube_open_left_transform = WALL_CUBE_OPEN_LEFT.clone();
    west_wall_cube_open_left_transform.rotate_around(Vec3::ZERO, west_quat);

    let north_wall_cube_open_right_transform = WALL_CUBE_OPEN_RIGHT.clone();
    let mut east_wall_cube_open_right_transform = WALL_CUBE_OPEN_RIGHT.clone();
    east_wall_cube_open_right_transform.rotate_around(Vec3::ZERO, east_quat);
    let mut south_wall_cube_open_right_transform = WALL_CUBE_OPEN_RIGHT.clone();
    south_wall_cube_open_right_transform.rotate_around(Vec3::ZERO, south_quat);
    let mut west_wall_cube_open_right_transform = WALL_CUBE_OPEN_RIGHT.clone();
    west_wall_cube_open_right_transform.rotate_around(Vec3::ZERO, west_quat);

    let north_place_short_closed_transform = PLACE_SHORT_CLOSED.clone();
    let mut east_place_short_closed_transform = PLACE_SHORT_CLOSED.clone();
    east_place_short_closed_transform.rotate_around(Vec3::ZERO, east_quat);
    let mut south_place_short_closed_transform = PLACE_SHORT_CLOSED.clone();
    south_place_short_closed_transform.rotate_around(Vec3::ZERO, south_quat);
    let mut west_place_short_closed_transform = PLACE_SHORT_CLOSED.clone();
    west_place_short_closed_transform.rotate_around(Vec3::ZERO, west_quat);

    let north_wall_long_closed_transform = WALL_LONG_CLOSED.clone();
    let mut east_wall_long_closed_transform = WALL_LONG_CLOSED.clone();
    east_wall_long_closed_transform.rotate_around(Vec3::ZERO, east_quat);
    let mut south_wall_long_closed_transform = WALL_LONG_CLOSED.clone();
    south_wall_long_closed_transform.rotate_around(Vec3::ZERO, south_quat);
    let mut west_wall_long_closed_transform = WALL_LONG_CLOSED.clone();
    west_wall_long_closed_transform.rotate_around(Vec3::ZERO, west_quat);

    println!(
        "Transform: {:?} {:?} {:?} {:?}",
        north_wall_cube_common_transform,
        east_wall_cube_common_transform,
        south_wall_cube_common_transform,
        west_wall_cube_common_transform
    );

    commands
        .spawn_empty()
        .insert((Transform::default(), Visibility::default()))
        .with_children(|parent| {
            for z in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    let z_pos = (z * CELL_HEIGHT) as f32;
                    let x_pos = (x * CELL_WIDTH) as f32;
                    parent
                        .spawn((
                            Mesh3d(ground_mesh_handle.clone()),
                            MeshMaterial3d(ground_material_handle.clone()),
                            Transform::from_xyz(x_pos, 0.0, z_pos),
                            Ground,
                            GridPos {
                                0: IVec2 {
                                    x: x as i32,
                                    y: z as i32,
                                },
                            },
                        ))
                        .with_children(|sub_parent| {
                            for direction in 0..4 {
                                match direction {
                                    0 => {}
                                    1 => {}
                                    2 => {}
                                    3 => {}
                                    _ => panic!("Shouldn't be here..."),
                                }
                            }
                            sub_parent.spawn((
                                Mesh3d(cube_mesh_handle.clone()),
                                MeshMaterial3d(wall_cube_material_handle.clone()),
                                north_wall_cube_common_transform,
                            ));
                            sub_parent.spawn((
                                Mesh3d(cube_mesh_handle.clone()),
                                MeshMaterial3d(wall_cube_material_handle.clone()),
                                east_wall_cube_common_transform,
                            ));
                            sub_parent.spawn((
                                Mesh3d(cube_mesh_handle.clone()),
                                MeshMaterial3d(wall_cube_material_handle.clone()),
                                south_wall_cube_common_transform,
                            ));
                            sub_parent.spawn((
                                Mesh3d(cube_mesh_handle.clone()),
                                MeshMaterial3d(wall_cube_material_handle.clone()),
                                west_wall_cube_common_transform,
                            ));
                            sub_parent
                                .spawn((
                                    Mesh3d(cube_mesh_handle.clone()),
                                    MeshMaterial3d(place_cube_material_handle.clone()),
                                    north_place_cube_common_transform,
                                ))
                                .observe(update_material_on::<Pointer<Over>>(
                                    place_cube_hover_handle.clone(),
                                ))
                                .observe(update_material_on::<Pointer<Out>>(
                                    place_cube_material_handle.clone(),
                                ))
                                .observe(update_material_on::<Pointer<Down>>(
                                    place_cube_pressed_handle.clone(),
                                ))
                                .observe(update_material_on::<Pointer<Up>>(
                                    place_cube_hover_handle.clone(),
                                ));
                            sub_parent.spawn((
                                Mesh3d(cube_mesh_handle.clone()),
                                MeshMaterial3d(place_cube_material_handle.clone()),
                                east_place_cube_common_transform,
                            ));
                            sub_parent.spawn((
                                Mesh3d(cube_mesh_handle.clone()),
                                MeshMaterial3d(place_cube_material_handle.clone()),
                                south_place_cube_common_transform,
                            ));
                            sub_parent.spawn((
                                Mesh3d(cube_mesh_handle.clone()),
                                MeshMaterial3d(place_cube_material_handle.clone()),
                                west_place_cube_common_transform,
                            ));

                            println!("Spawning North Tile for {} {}", x, z);
                            match maze.tiles[maze.idx((x as i32, z as i32).into())].north {
                                Exit::Open => {
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        north_wall_cube_open_left_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        north_wall_cube_open_right_transform,
                                    ));
                                }
                                Exit::Closed => {
                                    sub_parent.spawn((
                                        Mesh3d(short_wall_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        north_place_short_closed_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(long_wall_mesh_handle.clone()),
                                        MeshMaterial3d(place_cube_material_handle.clone()),
                                        north_wall_long_closed_transform,
                                    ));
                                }
                            }
                            println!("Spawning East Tile for {} {}", x, z);

                            match maze.tiles[maze.idx((x as i32, z as i32).into())].east {
                                Exit::Open => {
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        east_wall_cube_open_left_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        east_wall_cube_open_right_transform,
                                    ));
                                }
                                Exit::Closed => {
                                    sub_parent.spawn((
                                        Mesh3d(short_wall_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        east_place_short_closed_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(long_wall_mesh_handle.clone()),
                                        MeshMaterial3d(place_cube_material_handle.clone()),
                                        east_wall_long_closed_transform,
                                    ));
                                }
                            }

                            println!("Spawning South Tile for {} {}", x, z);

                            match maze.tiles[maze.idx((x as i32, z as i32).into())].south {
                                Exit::Open => {
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        south_wall_cube_open_left_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        south_wall_cube_open_right_transform,
                                    ));
                                }
                                Exit::Closed => {
                                    sub_parent.spawn((
                                        Mesh3d(short_wall_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        south_place_short_closed_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(long_wall_mesh_handle.clone()),
                                        MeshMaterial3d(place_cube_material_handle.clone()),
                                        south_wall_long_closed_transform,
                                    ));
                                }
                            }

                            match maze.tiles[maze.idx((x as i32, z as i32).into())].west {
                                Exit::Open => {
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        west_wall_cube_open_left_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        west_wall_cube_open_right_transform,
                                    ));
                                }
                                Exit::Closed => {
                                    sub_parent.spawn((
                                        Mesh3d(short_wall_mesh_handle.clone()),
                                        MeshMaterial3d(wall_cube_material_handle.clone()),
                                        west_place_short_closed_transform,
                                    ));
                                    sub_parent.spawn((
                                        Mesh3d(long_wall_mesh_handle.clone()),
                                        MeshMaterial3d(place_cube_material_handle.clone()),
                                        west_wall_long_closed_transform,
                                    ));
                                }
                            }
                        });
                }
            }
        });
}

fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}
