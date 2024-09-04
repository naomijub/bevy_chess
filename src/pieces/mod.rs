use bevy::prelude::*;
pub mod components;
pub mod helper;
mod setup;

#[derive(Resource, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::spawn_pieces)
            .add_systems(PostStartup, setup::add_pieces_name);
    }
}
