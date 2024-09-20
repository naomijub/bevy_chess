use crate::pieces::systems::move_pieces;
use bevy::prelude::*;
use components::{Piece, PieceColor, PieceType, Selected};
pub mod components;
pub mod helper;

mod setup;
mod systems;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selected>()
            .register_type::<Piece>()
            .register_type::<PieceColor>()
            .register_type::<PieceType>()
            .add_systems(Startup, setup::spawn_pieces)
            .add_systems(PostStartup, setup::add_pieces_name)
            .add_systems(Update, move_pieces);
    }
}
