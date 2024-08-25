use bevy::prelude::*;

use crate::board::components::Square;

#[derive(Clone, Copy, Eq, PartialEq, Reflect)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Eq, PartialEq, Reflect)]
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
        self.x == other.x && self.y == other.y
    }
}
