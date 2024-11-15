// use assets::AssetLoadingPlugin;
use crate::assets::GameAssets;
use assets::AssetLoadingPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use camera::CameraSetupPlugin;
use config::ConfigPlugin;
use input::ProcessInputPlugin;
use map::MapPlugin;
use selector::SelectorPlugin;

// mod assets;
mod assets;
mod camera;
mod config;
mod input;
mod map;
mod selector;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                    custom_layer: |_| None,
                }),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(ProcessInputPlugin)
        .add_plugins(AssetLoadingPlugin)
        .add_plugins(CameraSetupPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(SelectorPlugin)
        .add_plugins(ConfigPlugin)
        .run();
}
