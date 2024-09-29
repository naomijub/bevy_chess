use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use components::{PossibleMove, Square};
use systems::{despawn_taken, set_move_to_square, set_selections};

pub mod components;
mod setup;
mod systems;

#[derive(Debug, Clone, PartialEq, Eq, Event, Reflect)]
pub struct MoveToEvent {
    entity: Entity,
    to: (i8, i8),
}

#[derive(Debug, Clone, Reflect, Event)]
pub struct SelectedEvent(Entity);

#[derive(Debug, Clone, Reflect, Event)]
pub struct DespawnEvent(Entity);

#[derive(Default, Debug, Clone, Reflect, Resource)]
struct SelectedSquare {
    pub entity: Option<Entity>,
}

#[derive(Debug, Clone, Reflect, Resource)]
pub struct TilesHandles {
    pub white: Handle<StandardMaterial>,
    pub black: Handle<StandardMaterial>,

    pub mesh: Handle<Mesh>,
}

impl From<ListenerInput<Pointer<Down>>> for SelectedEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        Self(event.target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectedEvent>()
            .add_event::<MoveToEvent>()
            .add_event::<DespawnEvent>()
            .init_resource::<SelectedSquare>()
            .register_type::<TilesHandles>()
            .register_type::<SelectedEvent>()
            .register_type::<SelectedSquare>()
            .register_type::<Square>()
            .register_type::<PossibleMove>()
            .register_type::<MoveToEvent>()
            .register_type::<DespawnEvent>()
            .add_systems(Startup, setup::board)
            .add_systems(
                Update,
                (
                    set_selections.run_if(on_event::<SelectedEvent>()),
                    set_move_to_square.run_if(on_event::<MoveToEvent>()),
                    despawn_taken.run_if(on_event::<DespawnEvent>()),
                ),
            );
    }
}
