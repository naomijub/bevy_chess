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
    fn color_of(&self, square: &Square) -> Option<PieceColor>;
}

impl<'w, 's> Contains for Query<'w, 's, (Entity, &Piece)> {
    fn color_of(&self, square: &Square) -> Option<PieceColor> {
        self.iter().find(|p| p.1 == square).map(|p| p.1.color)
    }
}

pub fn is_path_empty(
    begin: &Square,
    end: &Square,
    pieces: &Query<'_, '_, (Entity, &Piece)>,
) -> bool {
    // Same column
    if begin.x == end.x
        && pieces.iter().any(|(_, piece)| {
            piece.x == begin.x
                && ((piece.y > begin.y && piece.y < end.y)
                    || (piece.y > end.y && piece.y < begin.y))
        })
    {
        return false;
    }

    // Same row
    if begin.y == end.y
        && pieces.iter().any(|(_, piece)| {
            piece.y == begin.y
                && ((piece.x > begin.x && piece.x < end.x)
                    || (piece.x > end.x && piece.x < begin.x))
        })
    {
        return false;
    }

    // Diagonals
    let x_diff = (begin.x - end.x).abs();
    let y_diff = (begin.y - end.y).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.x < end.x && begin.y < end.y {
                // left bottom - right top
                (begin.x + i, begin.y + i)
            } else if begin.x < end.x && begin.y > end.y {
                // left top - right bottom
                (begin.x + i, begin.y - i)
            } else if begin.x > end.x && begin.y < end.y {
                // right bottom - left top
                (begin.x - i, begin.y + i)
            } else {
                // begin.x > end.x && begin.y > end.y
                // right top - left bottom
                (begin.x - i, begin.y - i)
            };

            if pieces.color_of(&pos.into()).is_some() {
                return false;
            }
        }
    }

    true
}
