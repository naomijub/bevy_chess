use crate::board::components::Square;

use super::components::{PieceColor, PieceType};
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
