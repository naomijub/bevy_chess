use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use components::{PossibleMove, Square};
use systems::{define_possible_moves, set_selected_piece};

pub mod components;
mod setup;
mod systems;

#[derive(Debug, Clone, Reflect, Event)]
pub struct SelectedEvent(Entity);

#[derive(Debug, Clone, Reflect, Resource)]
pub struct TilesHandles {
    pub white: Handle<StandardMaterial>,
    pub black: Handle<StandardMaterial>,
    pub possible_move: Handle<StandardMaterial>,
    pub enemy_piece: Handle<StandardMaterial>,
    pub mesh: Handle<Mesh>,
}

impl From<ListenerInput<Pointer<Click>>> for SelectedEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectedEvent>()
            .register_type::<TilesHandles>()
            .register_type::<SelectedEvent>()
            .register_type::<Square>()
            .register_type::<PossibleMove>()
            .add_systems(Startup, setup::board)
            .add_systems(
                Update,
                (
                    set_selected_piece.before(define_possible_moves),
                    define_possible_moves,
                ),
            );
    }
}
