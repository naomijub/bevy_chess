use bevy::prelude::*;

use crate::{
    pieces::components::{Piece, PieceColor, PieceType},
    player::MoveType,
    ui::helpers::square_to_coord,
};

#[derive(Debug, Component, Clone, Reflect, Default)]
pub struct MovesUI(pub usize);

#[derive(Debug, Clone, Reflect)]
pub struct PlayerMove {
    pub color: PieceColor,
    pub piece: PieceType,
    pub origin: (i8, i8),
    pub destination: (i8, i8),
    pub move_type: MoveType,
}

impl PlayerMove {
    pub const fn castle(piece: &Piece, destination: (i8, i8)) -> Self {
        Self {
            color: piece.color,
            piece: piece.piece_type,
            origin: (piece.x, piece.y),
            destination,
            move_type: if destination.1 > piece.y {
                MoveType::CastlingRight
            } else {
                MoveType::CastlingLeft
            },
        }
    }

    pub const fn capture(piece: &Piece, destination: (i8, i8), other: &PieceType) -> Self {
        Self {
            color: piece.color,
            piece: piece.piece_type,
            origin: (piece.x, piece.y),
            destination,
            move_type: MoveType::Capture(*other),
        }
    }

    pub const fn promote(piece: &Piece, other: &PieceType) -> Self {
        Self {
            color: piece.color,
            piece: piece.piece_type,
            origin: (piece.x, piece.y),
            destination: (piece.x, piece.y),
            move_type: MoveType::Promotion(*other),
        }
    }

    pub const fn simple(piece: &Piece, destination: (i8, i8)) -> Self {
        Self {
            color: piece.color,
            piece: piece.piece_type,
            origin: (piece.x, piece.y),
            destination,
            move_type: MoveType::SimpleMove,
        }
    }

    pub const fn check(piece: &Piece, destination: (i8, i8)) -> Self {
        Self {
            color: piece.color,
            piece: piece.piece_type,
            origin: (piece.x, piece.y),
            destination,
            move_type: MoveType::SimpleCheck,
        }
    }
}

impl std::fmt::Display for PlayerMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}=>{} {}",
            self.color,
            self.piece,
            square_to_coord(&self.origin),
            square_to_coord(&self.destination),
            self.move_type
        )
    }
}

#[derive(Debug, Resource, Clone, Reflect, Default)]
pub struct PlayerMoves {
    pub moves: Vec<PlayerMove>,
}
