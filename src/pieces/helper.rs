use crate::board::components::Square;
use bevy::prelude::*;

use super::components::{Piece, PieceColor, PieceType};

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (1, 2),
    (-1, 2),
    (1, -2),
    (-1, -2),
    (2, 1),
    (-2, 1),
    (2, -1),
    (-2, -1),
];

const ROOK_OFFSETS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

const BISHOP_OFFSETS: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

const KING_OFFSETS: [(i8, i8); 8] = [
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

const WHITE_PAWN_OFFSETS: [(i8, i8); 4] = [(1, 1), (1, -1), (1, 0), (2, 0)];
const BLACK_PAWN_OFFSETS: [(i8, i8); 4] = [(-1, 1), (-1, -1), (-1, 0), (-2, 0)];

pub fn possible_moves(
    piece: PieceType,
    color: PieceColor,
    square: &Square,
    is_pawn_first_move: bool,
) -> Vec<Square> {
    let take_count = if is_pawn_first_move { 4 } else { 3 };
    match (piece, color) {
        (PieceType::Pawn, PieceColor::White) => WHITE_PAWN_OFFSETS
            .into_iter()
            .take(take_count)
            .map(|offset| square.clone() + offset)
            .filter(|square| square.inside_board())
            .collect(),
        (PieceType::Pawn, PieceColor::Black) => BLACK_PAWN_OFFSETS
            .into_iter()
            .take(take_count)
            .map(|offset| square.clone() + offset)
            .filter(|square| square.inside_board())
            .collect(),
        (PieceType::Knight, _) => KNIGHT_OFFSETS
            .into_iter()
            .map(|offset| square.clone() + offset)
            .filter(|square| square.inside_board())
            .collect(),
        (PieceType::King, _) => KING_OFFSETS
            .into_iter()
            .map(|offset| square.clone() + offset)
            .filter(|square| square.inside_board())
            .collect(),
        (PieceType::Rook, _) => (1..=8i8)
            .flat_map(|mul| {
                ROOK_OFFSETS
                    .into_iter()
                    .map(move |offset| square.clone() + (offset.0 * mul, offset.1 * mul))
            })
            .filter(|square| square.inside_board())
            .collect(),
        (PieceType::Bishop, _) => (1..=8i8)
            .flat_map(|mul| {
                BISHOP_OFFSETS
                    .into_iter()
                    .map(move |offset| square.clone() + (offset.0 * mul, offset.1 * mul))
            })
            .filter(|square| square.inside_board())
            .collect(),
        (PieceType::Queen, _) => (1..=8i8)
            .flat_map(|mul| {
                KING_OFFSETS
                    .into_iter()
                    .map(move |offset| square.clone() + (offset.0 * mul, offset.1 * mul))
            })
            .filter(|square| square.inside_board())
            .collect(),
    }
}

pub trait Contains {
    fn contains_any(&self, square: &Square) -> bool;
    fn contains_color(&self, square: &Square, color: &PieceColor) -> bool;
}

impl<'w, 's> Contains for Query<'w, 's, (Entity, &Piece)> {
    fn contains_any(&self, square: &Square) -> bool {
        self.iter().any(|(_, piece)| piece == square)
    }

    fn contains_color(&self, square: &Square, color: &PieceColor) -> bool {
        self.iter()
            .filter(|(_, piece)| piece.color == *color)
            .any(|(_, piece)| piece == square)
    }
}
