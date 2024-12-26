use std::f32::consts::TAU;

use bevy::color::palettes::tailwind::{CYAN_300, YELLOW_300};
//use bevy::picking::pointer::PointerInteraction;

use crate::prelude::*;

#[derive(Component)]
struct Ground;

#[derive(Component, Deref, DerefMut)]
struct GridPos(IVec2);

pub const WALL_CUBE_COMMON: Transform = Transform::from_xyz(1.5, 0.5, 1.5);
pub const PLACE_CUBE_COMMON: Transform = Transform::from_xyz(2.5, 0.5, 2.5);
pub const WALL_CUBE_OPEN_LEFT: Transform = Transform::from_xyz(-1.5, 0.5, 2.5);
pub const WALL_CUBE_OPEN_RIGHT: Transform = Transform::from_xyz(1.5, 0.5, 2.5);
pub const WALL_SHORT_CLOSED: Transform = Transform::from_xyz(0.0, 0.5, 1.5);
pub const PLACE_CLOSED_LEFT: Transform = Transform::from_xyz(-1.5, 0.5, 2.5);
pub const PLACE_CLOSED_MIDLEFT: Transform = Transform::from_xyz(-0.5, 0.5, 2.5);
pub const PLACE_CLOSED_MIDRIGHT: Transform = Transform::from_xyz(0.5, 0.5, 2.5);
pub const PLACE_CLOSED_RIGHT: Transform = Transform::from_xyz(1.5, 0.5, 2.5);

pub const TRANSFORM_ARRAY: [Transform; 9] = [
    WALL_CUBE_COMMON,
    PLACE_CUBE_COMMON,
    WALL_CUBE_OPEN_LEFT,
    WALL_CUBE_OPEN_RIGHT,
    WALL_SHORT_CLOSED,
    PLACE_CLOSED_LEFT,
    PLACE_CLOSED_MIDLEFT,
    PLACE_CLOSED_MIDRIGHT,
    PLACE_CLOSED_RIGHT,
];

pub fn generate_maze(mut commands: Commands) {
    let rng = thread_rng();
    let maze = Maze::new(6, 6, rng);
    commands.insert_resource(maze);
}

pub fn scene_setup(
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
    let short_wall_mesh_handle = meshes.add(Cuboid::new(2., 1., 1.));

    let quaternion_array = [
        Quat::from_axis_angle(Vec3::Y, 0.),
        Quat::from_axis_angle(Vec3::Y, TAU / 4.0),
        Quat::from_axis_angle(Vec3::Y, TAU / 2.0),
        Quat::from_axis_angle(Vec3::Y, (TAU / 4.0) * 3.),
    ];

    let rotated_transform_array: Vec<Transform> = TRANSFORM_ARRAY
        .iter()
        .flat_map(|transform| {
            quaternion_array.iter().map(move |quaternion| {
                let mut new_transform = *transform;
                new_transform.rotate_around(Vec3::ZERO, *quaternion);
                new_transform
            })
        })
        .collect();

    println!("{:#?}", rotated_transform_array);

    println!("{}", TRANSFORM_ARRAY.len());

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
                                sub_parent.spawn((
                                    Mesh3d(cube_mesh_handle.clone()),
                                    MeshMaterial3d(wall_cube_material_handle.clone()),
                                    rotated_transform_array[direction],
                                ));
                                sub_parent
                                    .spawn((
                                        Mesh3d(cube_mesh_handle.clone()),
                                        MeshMaterial3d(place_cube_material_handle.clone()),
                                        rotated_transform_array[direction + 4],
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
                                    ))
                                    .insert(RayCastPickable);
                                match maze.tiles[maze.idx((x as i32, z as i32).into())].exits
                                    [direction]
                                {
                                    Exit::Open => {
                                        sub_parent.spawn((
                                            Mesh3d(cube_mesh_handle.clone()),
                                            MeshMaterial3d(wall_cube_material_handle.clone()),
                                            rotated_transform_array[direction + 8],
                                        ));
                                        sub_parent.spawn((
                                            Mesh3d(cube_mesh_handle.clone()),
                                            MeshMaterial3d(wall_cube_material_handle.clone()),
                                            rotated_transform_array[direction + 12],
                                        ));
                                    }
                                    Exit::Closed => {
                                        sub_parent
                                            .spawn((
                                                Mesh3d(cube_mesh_handle.clone()),
                                                MeshMaterial3d(place_cube_material_handle.clone()),
                                                rotated_transform_array[direction + 20],
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
                                            ))
                                            .insert(RayCastPickable);
                                        sub_parent
                                            .spawn((
                                                Mesh3d(cube_mesh_handle.clone()),
                                                MeshMaterial3d(place_cube_material_handle.clone()),
                                                rotated_transform_array[direction + 24],
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
                                            ))
                                            .insert(RayCastPickable);
                                        sub_parent
                                            .spawn((
                                                Mesh3d(cube_mesh_handle.clone()),
                                                MeshMaterial3d(place_cube_material_handle.clone()),
                                                rotated_transform_array[direction + 28],
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
                                            ))
                                            .insert(RayCastPickable);
                                        sub_parent
                                            .spawn((
                                                Mesh3d(cube_mesh_handle.clone()),
                                                MeshMaterial3d(place_cube_material_handle.clone()),
                                                rotated_transform_array[direction + 32],
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
                                            ))
                                            .insert(RayCastPickable);
                                        sub_parent.spawn((
                                            Mesh3d(short_wall_mesh_handle.clone()),
                                            MeshMaterial3d(wall_cube_material_handle.clone()),
                                            rotated_transform_array[direction + 16],
                                        ));
                                    }
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
