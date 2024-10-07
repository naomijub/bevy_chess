use bevy::prelude::*;

use super::components::{PieceColor, PieceType};

#[derive(Default, Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub struct PromotionChoice {
    pub entity: Option<Entity>,
    pub color: Option<PieceColor>,
    pub piece_type: Option<PieceType>,
}

impl PromotionChoice {
    pub const fn is_empty(&self) -> bool {
        self.entity.is_none() && self.color.is_none() && self.piece_type.is_none()
    }

    pub const fn can_choose(&self) -> bool {
        self.entity.is_some() && self.color.is_some() && self.piece_type.is_none()
    }

    pub const fn is_set(&self) -> bool {
        self.entity.is_some() && self.color.is_some() && self.piece_type.is_some()
    }

    pub fn clear(&mut self) {
        self.entity = None;
        self.color = None;
        self.piece_type = None;
    }
}

#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct Sprites {
    pub atlas: Handle<TextureAtlasLayout>,
    pub image: UiImage,
}

#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct PieceHandles {
    pub black_material: Handle<StandardMaterial>,
    pub white_material: Handle<StandardMaterial>,
    pub king_handle: Handle<Mesh>,
    pub king_cross_handle: Handle<Mesh>,
    pub pawn_handle: Handle<Mesh>,
    pub knight_1_handle: Handle<Mesh>,
    pub knight_2_handle: Handle<Mesh>,
    pub rook_handle: Handle<Mesh>,
    pub bishop_handle: Handle<Mesh>,
    pub queen_handle: Handle<Mesh>,
}
