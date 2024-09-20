use bevy::prelude::*;

use crate::board::components::Square;

use super::helper::{is_path_empty, Contains};

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

#[derive(Debug, Clone, Component, Eq, PartialEq, Reflect)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub x: u8,
    pub y: u8,
    pub first_move: bool,
}

impl PartialEq<Square> for Piece {
    fn eq(&self, other: &Square) -> bool {
        self.x as i16 == other.x as i16 && self.y as i16 == other.y as i16
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

impl Piece {
    pub fn name(&self) -> Name {
        Name::new(format!("{:?} {:?}", self.piece_type, self.color))
    }
    pub fn is_move_valid(
        &self,
        new_position: &Square,
        pieces: &Query<'_, '_, (Entity, &Self)>,
    ) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if pieces.color_of(new_position) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.x as i8 - new_position.x).abs() == 1
                && (self.y == new_position.y as u8))
                // Vertical
                || ((self.y as i8 - new_position.y).abs() == 1
                    && (self.x == new_position.x as u8))
                // Diagonal
                || ((self.x as i8 - new_position.x).abs() == 1
                    && (self.y as i8 - new_position.y).abs() == 1)
            }
            PieceType::Queen => {
                is_path_empty(&(self.x, self.y).into(), new_position, pieces)
                    && ((self.x as i8 - new_position.x).abs()
                        == (self.y as i8 - new_position.y).abs()
                        || ((self.x == new_position.x as u8 && self.y != new_position.y as u8)
                            || (self.y == new_position.y as u8 && self.x != new_position.x as u8)))
            }
            PieceType::Bishop => {
                is_path_empty(&(self.x, self.y).into(), new_position, pieces)
                    && (self.x as i8 - new_position.x).abs()
                        == (self.y as i8 - new_position.y).abs()
            }
            PieceType::Knight => {
                ((self.x as i8 - new_position.x).abs() == 2
                    && (self.y as i8 - new_position.y).abs() == 1)
                    || ((self.x as i8 - new_position.x).abs() == 1
                        && (self.y as i8 - new_position.y).abs() == 2)
            }
            PieceType::Rook => {
                is_path_empty(&(self.x, self.y).into(), new_position, pieces)
                    && ((self.x == new_position.x as u8 && self.y != new_position.y as u8)
                        || (self.y == new_position.y as u8 && self.x != new_position.x as u8))
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    // Normal move
                    if new_position.x as u8 - self.x == 1
                        && (self.y == new_position.y as u8)
                        && pieces.color_of(new_position).is_none()
                    {
                        return true;
                    }

                    // Move 2 squares
                    if self.x == 1
                        && new_position.x - self.x as i8 == 2
                        && (self.y == new_position.y as u8)
                        && is_path_empty(&(self.x, self.y).into(), new_position, pieces)
                        && pieces.color_of(new_position).is_none()
                    {
                        return true;
                    }

                    // Take piece
                    if new_position.x - self.x as i8 == 1
                        && (self.y as i8 - new_position.y).abs() == 1
                        && pieces.color_of(new_position) == Some(PieceColor::Black)
                    {
                        return true;
                    }
                } else {
                    // Normal move
                    if new_position.x - self.x as i8 == -1
                        && (self.y == new_position.y as u8)
                        && pieces.color_of(new_position).is_none()
                    {
                        return true;
                    }

                    // Move 2 squares
                    if self.x == 6
                        && new_position.x - self.x as i8 == -2
                        && (self.y == new_position.y as u8)
                        && is_path_empty(&(self.x, self.y).into(), new_position, pieces)
                        && pieces.color_of(new_position).is_none()
                    {
                        return true;
                    }

                    // Take piece
                    if new_position.x - self.x as i8 == -1
                        && (self.y as i8 - new_position.y).abs() == 1
                        && pieces.color_of(new_position) == Some(PieceColor::White)
                    {
                        return true;
                    }
                }

                false
            }
        }
    }
}
