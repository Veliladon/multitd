use bevy::prelude::*;
// use leafwing_input_manager::prelude::MouseScrollAxis;

#[derive(Resource)]
/* pub struct Config {
    pub zoom: MouseScrollAxis,
}

impl Config {
    fn new() -> Config {
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            zoom: MouseScrollAxis::Y,
        }
    }
} */

/* #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ConfigLoading; */

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_config)
            .add_systems(PreStartup, parse_config.after(load_config));
    }
}

fn load_config() {}

fn parse_config() {
    // commands.insert_resource(game_config);
}
