use crate::prelude::*;

#[derive(Component)]
pub struct EnemySpawner {
    pub timer: Timer,
    pub counter: usize,
}

#[derive(Resource)]
pub struct EnemyMeshHandle {
    pub cube_mesh_handle: Handle<Mesh>,
    pub cube_material_handle: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Mobile {
    pub speed: f32,
    pub destination: usize,
    pub direction: IVec2,
    pub index: usize,
}

#[derive(Component, DerefMut, Deref)]
pub struct FollowsLane(pub usize);

#[derive(Component)]
pub struct ReachedGoal;
