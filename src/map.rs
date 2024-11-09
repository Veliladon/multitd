use super::*;
use bevy::color::palettes::css::*;

#[derive(Component)]
struct Ground;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scene_setup);
    }
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("4way.glb")),
        transform: Transform::from_xyz(3., 0., 0.),
        ..default()
    });

    // 3-Way Junction
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("3way.glb")),
        transform: Transform::from_xyz(9., 0., 0.),
        ..default()
    });

    // 2-Way Junction

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("2way.glb")),
        transform: Transform::from_xyz(15., 0., 0.),
        ..default()
    });

    // End Cap
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("endcap.glb")),
        transform: Transform::from_xyz(3., 0., 6.),
        ..default()
    });
}

/* fn test_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("4way.glb")),
        transform: Transform::from_xyz(3., 0., 0.),
        ..default()
    });

    // 3-Way Junction
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("3way.glb")),
        transform: Transform::from_xyz(9., 0., 0.),
        ..default()
    });

    // 2-Way Junction

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("2way.glb")),
        transform: Transform::from_xyz(15., 0., 0.),
        ..default()
    });

    // End Cap
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("endcap.glb")),
        transform: Transform::from_xyz(3., 0., 6.),
        ..default()
    });
} */
