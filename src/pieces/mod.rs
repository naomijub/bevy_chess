use crate::pieces::systems::move_pieces;
use bevy::prelude::*;
use components::{Piece, PieceColor, PieceType, Selected};
use resources::{PromotionChoice, Sprites};
use systems::{button_system, promote, promote_pawn_to, promotion_ui};
pub mod components;
pub mod helper;

mod resources;
mod setup;
mod systems;

#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct CanPromoteEvent(pub Entity);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selected>()
            .register_type::<Piece>()
            .register_type::<PieceColor>()
            .register_type::<PieceType>()
            .register_type::<PromotionChoice>()
            .register_type::<Sprites>()
            .init_resource::<PromotionChoice>()
            .add_event::<CanPromoteEvent>()
            .add_systems(Startup, (setup::spawn_pieces, setup::load_sprites))
            .add_systems(PostStartup, setup::add_pieces_name)
            .add_systems(
                Update,
                (
                    move_pieces,
                    promote,
                    promotion_ui,
                    button_system,
                    promote_pawn_to,
                ),
            );
    }
}
