use bevy::prelude::*;

use crate::board::components::Square;

#[derive(Debug, Clone, Component, Reflect)]
pub struct Selected {
    pub color: PieceColor,
    pub piece_type: PieceType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Component, Eq, PartialEq, Reflect)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub x: u8,
    pub y: u8,
}

impl PartialEq<Square> for Piece {
    fn eq(&self, other: &Square) -> bool {
        self.x as i16 == other.x as i16 && self.y as i16 == other.y as i16
    }
}

impl Piece {
    pub fn name(&self) -> Name {
        Name::new(format!("{:?} {:?}", self.piece_type, self.color))
    }
}

impl PieceColor {
    pub const fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
