pub use crate::prelude::*;

#[derive(Resource)]
pub struct TowerAssets {
    pub cylinder_handle: Handle<Mesh>,
    pub cylinder_material: Handle<StandardMaterial>,
}
