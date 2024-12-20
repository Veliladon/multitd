use crate::prelude::*;

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect, Deref, DerefMut)]
pub struct CursorPiece(pub MazePieces);

/* #[derive(Component)]
pub struct CursorEntity; */

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MazePieces {
    #[default]
    FourWay,
    ThreeWay,
    TwoWay,
    EndCap,
}

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_selections)
            .add_systems(Startup, setup_selection)
            .add_systems(Update, process_move)
            .init_state::<MazePieces>()
            .register_type::<MazePieces>()
            .init_resource::<CursorPiece>()
            .register_type::<CursorPiece>();
    }
}

pub fn setup_selection() {}

pub fn process_move() {}

pub fn process_selections() {}
