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
    pub x: i8,
    pub y: i8,
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
    pub const fn is_diagonal(&self, square: &Square) -> bool {
        (self.x - square.x).abs() == (self.y - square.y).abs()
    }

    pub fn name(&self) -> Name {
        Name::new(format!("{:?} {:?}", self.piece_type, self.color))
    }

    pub fn is_king(&self) -> bool {
        self.piece_type == PieceType::King
    }

    pub fn is_move_valid(
        &self,
        new_position: &Square,
        pieces: &Query<'_, '_, (Entity, &Self)>,
    ) -> (bool, Option<(Entity, i8)>) {
        // If there's a piece of the same color in the same square, it can't move
        if pieces.color_of(new_position) == Some(self.color) {
            return (false, None);
        }

        match self.piece_type {
            PieceType::King => {
                let castling = self.can_castle(new_position, pieces);
                (self.king_moves(new_position) || castling.0, castling.1)
            }
            PieceType::Queen => (self.queen_moves(new_position, pieces), None),
            PieceType::Bishop => (self.bishop_moves(new_position, pieces), None),
            PieceType::Knight => (self.knight_moves(new_position), None),
            PieceType::Rook => (self.rook_moves(new_position, pieces), None),
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    (self.white_pawn_move(new_position, pieces), None)
                } else {
                    (self.black_pawn_move(new_position, pieces), None)
                }
            }
        }
    }

    fn black_pawn_move(
        &self,
        new_position: &Square,
        pieces: &Query<'_, '_, (Entity, &Self)>,
    ) -> bool {
        // Normal move
        if new_position.x - self.x == -1
            && (self.y == new_position.y)
            && pieces.color_of(new_position).is_none()
        {
            return true;
        }

        // Move 2 squares for first move
        if self.first_move
            && new_position.x - self.x == -2
            && (self.y == new_position.y)
            && is_path_empty(&(self.x, self.y).into(), new_position, pieces)
            && pieces.color_of(new_position).is_none()
        {
            return true;
        }

        // Take piece
        if new_position.x - self.x == -1
            && (self.y - new_position.y).abs() == 1
            && pieces.color_of(new_position) == Some(PieceColor::White)
        {
            return true;
        }
        false
    }

    fn white_pawn_move(
        &self,
        new_position: &Square,
        pieces: &Query<'_, '_, (Entity, &Self)>,
    ) -> bool {
        // Normal move
        if new_position.x - self.x == 1
            && (self.y == new_position.y)
            && pieces.color_of(new_position).is_none()
        {
            return true;
        }

        // Move 2 squares for first move
        if self.first_move
            && new_position.x - self.x == 2
            && (self.y == new_position.y)
            && is_path_empty(&(self.x, self.y).into(), new_position, pieces)
            && pieces.color_of(new_position).is_none()
        {
            return true;
        }

        // Take piece
        if new_position.x - self.x == 1
            && (self.y - new_position.y).abs() == 1
            && pieces.color_of(new_position) == Some(PieceColor::Black)
        {
            return true;
        }
        false
    }

    fn rook_moves(&self, new_position: &Square, pieces: &Query<'_, '_, (Entity, &Self)>) -> bool {
        is_path_empty(&(self.x, self.y).into(), new_position, pieces)
            && ((self.x == new_position.x && self.y != new_position.y)
                || (self.y == new_position.y && self.x != new_position.x))
    }

    const fn knight_moves(&self, new_position: &Square) -> bool {
        ((self.x - new_position.x).abs() == 2 && (self.y - new_position.y).abs() == 1)
            || ((self.x - new_position.x).abs() == 1 && (self.y - new_position.y).abs() == 2)
    }

    fn bishop_moves(&self, new_position: &Square, pieces: &Query<'_, '_, (Entity, &Self)>) -> bool {
        is_path_empty(&(self.x, self.y).into(), new_position, pieces)
            && (self.x - new_position.x).abs() == (self.y - new_position.y).abs()
    }

    fn queen_moves(&self, new_position: &Square, pieces: &Query<'_, '_, (Entity, &Self)>) -> bool {
        is_path_empty(&(self.x, self.y).into(), new_position, pieces)
            && ((self.x - new_position.x).abs() == (self.y - new_position.y).abs()
                || ((self.x == new_position.x && self.y != new_position.y)
                    || (self.y == new_position.y && self.x != new_position.x)))
    }

    const fn king_moves(&self, new_position: &Square) -> bool {
        // Horizontal
        ((self.x  - new_position.x).abs() == 1
        && (self.y == new_position.y ))
        // Vertical
        || ((self.y  - new_position.y).abs() == 1
            && (self.x == new_position.x ))
        // Diagonal
        || ((self.x  - new_position.x).abs() == 1
            && (self.y  - new_position.y).abs() == 1)
    }

    const CASTLE_COLUMNS: [(i8, i8, i8); 2] = [(1, 0, 2), (6, 7, 5)];
    fn can_castle(
        &self,
        new_position: &Square,
        pieces: &Query<'_, '_, (Entity, &Self)>,
    ) -> (bool, Option<(Entity, i8)>) {
        let castle_rook = Self::CASTLE_COLUMNS
            .iter()
            .find(|pos| (new_position.x == 0 || new_position.x == 7) && pos.0 == new_position.y);

        if self.first_move && castle_rook.is_some() {
            let Some((rook_entity, rook)) = pieces
                .iter()
                .filter(|(_, piece)| {
                    piece.piece_type == PieceType::Rook && piece.color == self.color
                })
                .find(|(_, piece)| piece.y == castle_rook.unwrap().1)
            else {
                return (false, None);
            };

            if rook.first_move {
                (
                    is_path_empty(&(self.x, self.y).into(), new_position, pieces),
                    Some((rook_entity, castle_rook.unwrap().2)),
                )
            } else {
                (false, None)
            }
        } else {
            (false, None)
        }
    }
}
