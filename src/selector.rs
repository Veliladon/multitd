use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use std::f32::consts::TAU;

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_selections);
    }
}

pub fn process_selections() {}
