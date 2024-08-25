use bevy::prelude::*;

#[derive(Debug, Clone, Reflect, PartialEq, Eq, Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub const fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}
