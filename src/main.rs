// mod assets;
mod assets;
mod camera;
mod config;
mod enemy;
mod input;
mod maze;
mod selector;
mod utils;

mod prelude {

    // use assets::AssetLoadingPlugin;
    pub use crate::assets::*;
    pub use crate::camera::*;
    pub use crate::config::*;
    pub use crate::enemy::*;
    pub use crate::input::*;
    pub use crate::maze::*;
    pub use crate::selector::*;
    pub use crate::utils::*;
    pub use bevy::log::LogPlugin;
    pub use bevy::prelude::*;
}

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    text::FontSmoothing,
};
use prelude::*;

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
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 42.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                },
                // We can also change color of the overlay
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
            },
        })
        //.add_plugins(WorldInspectorPlugin::new())
        /*        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..default()
        }) */
        .add_plugins(MeshPickingPlugin)
        .add_plugins(ProcessInputPlugin)
        .add_plugins(AssetLoadingPlugin)
        .add_plugins(CameraSetupPlugin)
        .add_plugins(MazePlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(SelectorPlugin)
        .add_plugins(ConfigPlugin)
        .run();
}
