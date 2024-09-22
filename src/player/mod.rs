use bevy::prelude::*;
use systems::victory_screen;

mod systems;

use crate::pieces::components::{Piece, PieceColor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Turn>()
            .register_type::<Turn>()
            .init_resource::<SelectedPlayerPiece>()
            .register_type::<SelectedPlayerPiece>()
            .add_event::<VictoryEvent>()
            .register_type::<VictoryEvent>()
            .add_systems(Update, victory_screen.run_if(on_event::<VictoryEvent>()));
    }
}

#[derive(Debug, Clone, Reflect, Event)]
pub struct VictoryEvent(pub PieceColor);

#[derive(Default, Debug, Clone, Reflect, Resource)]
pub struct SelectedPlayerPiece {
    pub entity: Option<Entity>,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Reflect, Resource)]
pub enum Turn {
    #[default]
    White,
    Black,
    End,
}

impl PartialEq<&Piece> for Turn {
    fn eq(&self, other: &&Piece) -> bool {
        matches!(
            (other.color, self),
            (PieceColor::White, Self::White) | (PieceColor::Black, Self::Black)
        )
    }
}

impl From<PieceColor> for Turn {
    fn from(color: PieceColor) -> Self {
        match color {
            PieceColor::White => Self::White,
            PieceColor::Black => Self::Black,
        }
    }
}

impl From<Turn> for PieceColor {
    fn from(color: Turn) -> Self {
        match color {
            Turn::White => Self::White,
            Turn::Black => Self::Black,
            Turn::End => Self::White,
        }
    }
}
