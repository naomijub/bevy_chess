use bevy::prelude::*;

use crate::pieces::components::{Piece, Selected};

use super::{components::Square, SelectedEvent, TilesHandles};

pub fn set_selected_piece(
    mut events: EventReader<SelectedEvent>,
    mut commands: Commands,
    pieces: Query<(Entity, &Piece)>,
    tiles: Query<(Entity, &Square)>,
    previous_selected: Query<Entity, With<Selected>>,
) {
    let Some(&SelectedEvent(selected_entity)) = events.read().next() else {
        return;
    };

    previous_selected.iter().for_each(|entity| {
        commands.entity(entity).remove::<Selected>();
    });

    let Ok((tile, square)) = tiles.get(selected_entity) else {
        return;
    };

    let Some(selected_piece) = pieces.iter().find(|(_, piece)| piece == &square) else {
        return;
    };

    commands.entity(selected_piece.0).insert(Selected {
        color: selected_piece.1.color,
        piece_type: selected_piece.1.piece_type,
    });
    commands.entity(tile).insert(Selected {
        color: selected_piece.1.color,
        piece_type: selected_piece.1.piece_type,
    });
}

pub fn define_possible_moves(
    mut tiles: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
    selected_pieces: Query<(Entity, &Square, &Selected)>,
    tiles_handle: Res<TilesHandles>,
) {
    let Ok((selected_entity, square, selected)) = selected_pieces.get_single() else {
        return;
    };

    let possible_moves =
        crate::pieces::helper::possible_moves(selected.piece_type, selected.color, square);

    for (entity, tile, mut material) in tiles.iter_mut() {
        if entity == selected_entity {
            continue;
        }
        if possible_moves.contains(tile) {
            *material = tiles_handle.possible_move.clone();
        } else {
            *material = if tile.is_white() {
                tiles_handle.white.clone()
            } else {
                tiles_handle.black.clone()
            };
        }
    }
}
