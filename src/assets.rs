use bevy::prelude::*;

pub struct AssetLoadingPlugin;

#[derive(Resource)]
pub struct GameAssets {
    pub fourway_handle: Handle<Scene>,
    pub threeway_handle: Handle<Scene>,
    pub twoway_handle: Handle<Scene>,
    pub endcap_handle: Handle<Scene>,
    pub ground_material_handle: Handle<StandardMaterial>,
}

pub const FOURWAY: &str = "4way.glb";
pub const THREEWAY: &str = "3way.glb";
pub const TWOWAY: &str = "2way.glb";
pub const ENDCAP: &str = "endcap.glb";

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let fourway_handle: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(FOURWAY));
    let threeway_handle: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(THREEWAY));
    let twoway_handle: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(TWOWAY));
    let endcap_handle: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(ENDCAP));
    let ground_material_handle = materials.add(Color::srgb(0.3, 0.5, 0.0));

    commands.insert_resource(GameAssets {
        fourway_handle,
        threeway_handle,
        twoway_handle,
        endcap_handle,
        ground_material_handle,
    });
}
