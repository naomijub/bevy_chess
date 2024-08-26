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

pub fn possible_moves(piece: PieceType, color: PieceColor, square: &Square) -> Vec<Square> {
    match (piece, color) {
        (PieceType::Knight, _) => KNIGHT_OFFSETS
            .iter()
            .map(|offset| square.clone() + *offset)
            .filter(|square| square.inside_board())
            .collect(),
        _ => Vec::new(),
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
