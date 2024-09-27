use std::ops::Add;

use bevy::prelude::*;

use crate::pieces::components::Piece;

#[derive(Debug, Clone, Reflect, PartialEq, Eq, Component)]
pub struct Square {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, Component)]
pub enum PossibleMove {
    Enemy,
    Empty,
}

impl Square {
    pub const fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }

    pub const fn inside_board(&self) -> bool {
        self.x >= 0 && self.x < 8 && self.y >= 0 && self.y < 8
    }
}

impl Add<(i8, i8)> for Square {
    type Output = Self;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl From<(i8, i8)> for Square {
    fn from((x, y): (i8, i8)) -> Self {
        Self { x, y }
    }
}

impl From<Square> for (i8, i8) {
    fn from(square: Square) -> Self {
        (square.x, square.y)
    }
}

impl From<&Square> for (i8, i8) {
    fn from(square: &Square) -> Self {
        (square.x, square.y)
    }
}

impl From<&Piece> for Square {
    fn from(piece: &Piece) -> Self {
        Self {
            x: piece.x,
            y: piece.y,
        }
    }
}
