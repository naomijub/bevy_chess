use bevy::prelude::*;
use setup::add_turn_text;
use systems::{draw_possible_moves, text_update_system, victory_screen};

mod setup;
mod systems;

use crate::pieces::components::{Piece, PieceColor, PieceType};

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
            .add_systems(Startup, add_turn_text)
            .add_systems(
                Update,
                (
                    text_update_system,
                    draw_possible_moves,
                    victory_screen.run_if(on_event::<VictoryEvent>()),
                ),
            );
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

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "White"),
            Self::Black => write!(f, "Black"),
            Self::End => write!(f, "End"),
        }
    }
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
pub enum MoveType {
    SimpleMove,
    Capture(PieceType),
    EnPassant,
    CastlingRight,
    CastlingLeft,
    Promotion(PieceType),
    SimpleCheck,
}

impl std::fmt::Display for MoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SimpleMove => String::new(),
                Self::Capture(piece) => format!("Capture: {}", piece),
                Self::EnPassant => "EnPassant".to_string(),
                Self::CastlingRight => "Short Castle".to_string(),
                Self::CastlingLeft => "Long Castle".to_string(),
                Self::Promotion(piece) => format!("Promotion to: {}", piece),
                Self::SimpleCheck => "Check".to_string(),
            }
        )
    }
}
