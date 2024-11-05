use crate::*;

pub struct AssetLoadingPlugin;

#[derive(Resource)]
pub struct GameAssets {
    pub game_texture: Handle<Image>,
    pub game_tile: Handle<TextureAtlasLayout>,
}

pub const TD_ASSETS: &str = "tdassets.png";

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let asset_handle: Handle<Image> = asset_server.load(TD_ASSETS);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.insert_resource(GameAssets {
        game_texture: asset_handle,
        game_tile: texture_atlas_layout,
    });
}
