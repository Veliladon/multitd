// mod assets;
mod assets;
mod camera;
mod config;
mod input;
mod maze;
mod selector;

mod prelude {

    // use assets::AssetLoadingPlugin;
    pub use crate::assets::*;
    pub use crate::camera::*;
    pub use crate::config::*;
    pub use crate::input::*;
    pub use crate::maze::*;
    pub use crate::selector::*;
    pub use bevy::log::LogPlugin;
    pub use bevy::prelude::*;
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use bevy_mod_picking::DefaultPickingPlugins;
}

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
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(ProcessInputPlugin)
        .add_plugins(AssetLoadingPlugin)
        .add_plugins(CameraSetupPlugin)
        .add_plugins(MazePlugin)
        .add_plugins(SelectorPlugin)
        .add_plugins(ConfigPlugin)
        .run();
}
