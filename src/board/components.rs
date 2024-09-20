use std::ops::Add;

use bevy::prelude::*;

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

impl Add<(u8, u8)> for Square {
    type Output = Self;

    fn add(self, rhs: (u8, u8)) -> Self::Output {
        Self {
            x: self.x + rhs.0 as i8,
            y: self.y + rhs.1 as i8,
        }
    }
}

impl From<(u8, u8)> for Square {
    fn from((x, y): (u8, u8)) -> Self {
        Self {
            x: x as i8,
            y: y as i8,
        }
    }
}

impl From<(i8, i8)> for Square {
    fn from((x, y): (i8, i8)) -> Self {
        Self { x, y }
    }
}

impl From<Square> for (u8, u8) {
    fn from(square: Square) -> Self {
        (square.x as u8, square.y as u8)
    }
}

impl From<&Square> for (u8, u8) {
    fn from(square: &Square) -> Self {
        (square.x as u8, square.y as u8)
    }
}
