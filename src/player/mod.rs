use bevy::prelude::*;

use crate::pieces::components::{Piece, PieceColor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Turn>().register_type::<Turn>();
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Reflect, Resource)]
pub enum Turn {
    #[default]
    White,
    Black,
}

impl PartialEq<&Piece> for Turn {
    fn eq(&self, other: &&Piece) -> bool {
        matches!(
            (other.color, self),
            (PieceColor::White, Self::White) | (PieceColor::Black, Self::Black)
        )
    }
}
