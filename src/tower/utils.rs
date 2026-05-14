pub use crate::prelude::*;

#[derive(Resource)]
pub struct TowerAssets {
    pub cylinder_handle: Handle<Mesh>,
    pub cylinder_material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct TowerStats {
    pub damage: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub target: Entity,
}
