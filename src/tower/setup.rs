use bevy::color::palettes::css::GREY;

pub use crate::prelude::*;

pub fn generate_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cylinder_handle = meshes.add(Cylinder::default());
    let cylinder_material = materials.add(Color::from(GREY));

    let tower_assets = TowerAssets {
        cylinder_handle,
        cylinder_material,
    };
    commands.insert_resource(tower_assets)
}
