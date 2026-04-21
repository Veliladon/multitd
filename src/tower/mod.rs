pub use crate::prelude::*;
pub mod setup;
pub mod utils;

pub use setup::*;
pub use utils::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_assets);
    }
}
