use bevy::prelude::*;

use crate::{
    pieces::{
        components::{Piece, PieceType, Selected},
        helper::Contains,
    },
    player::Turn,
};

use super::{
    components::{PossibleMove, Square},
    SelectedEvent, TilesHandles,
};

pub fn set_selected_piece(
    mut events: EventReader<SelectedEvent>,
    mut commands: Commands,
    pieces: Query<(Entity, &Piece)>,
    mut tiles: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
    previous_selected: Query<Entity, With<Selected>>,
    tiles_handle: Res<TilesHandles>,
    turn: Res<Turn>,
) {
    let Some(&SelectedEvent(selected_entity)) = events.read().next() else {
        return;
    };

    previous_selected.iter().for_each(|entity| {
        commands.entity(entity).remove::<Selected>();
    });

    for (entity, tile, mut material) in tiles.iter_mut() {
        if entity == selected_entity {
            continue;
        }
        *material = if tile.is_white() {
            tiles_handle.white.clone()
        } else {
            tiles_handle.black.clone()
        }
    }

    let Ok((selected_tile, square, _)) = tiles.get(selected_entity) else {
        return;
    };

    let Some(selected_piece) = pieces.iter().find(|(_, piece)| piece == &square) else {
        return;
    };

    if *turn != selected_piece.1 {
        return;
    }

    commands.entity(selected_piece.0).insert(Selected {
        color: selected_piece.1.color,
        piece_type: selected_piece.1.piece_type,
    });
    commands.entity(selected_tile).insert(Selected {
        color: selected_piece.1.color,
        piece_type: selected_piece.1.piece_type,
    });
}

pub fn define_possible_moves(
    mut commands: Commands,
    mut tiles: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
    board_pieces: Query<(Entity, &Piece)>,
    selected_pieces: Query<(Entity, &Square, &Selected)>,
    previous_possible_moves: Query<Entity, With<PossibleMove>>,
    tiles_handle: Res<TilesHandles>,
) {
    let Ok((selected_entity, square, selected)) = selected_pieces.get_single() else {
        return;
    };

    let can_pawn = board_pieces
        .iter()
        .filter(|(_, piece)| piece.piece_type == PieceType::Pawn)
        .find(|(_, piece)| piece == &square)
        .map(|(_, piece)| piece);

    let possible_moves = crate::pieces::helper::possible_moves(
        selected.piece_type,
        selected.color,
        square,
        can_pawn.map(|piece| piece.first_move).unwrap_or_default(),
    );

    previous_possible_moves.iter().for_each(|entity| {
        commands.entity(entity).remove::<PossibleMove>();
    });

    for (entity, tile, mut material) in tiles.iter_mut() {
        if entity == selected_entity {
            continue;
        }
        if possible_moves.contains(tile) {
            if board_pieces.contains_color(tile, &selected.color.opposite()) {
                commands.entity(entity).insert(PossibleMove::Enemy);
                *material = tiles_handle.enemy_piece.clone();
            } else if board_pieces.contains_color(tile, &selected.color) {
                continue;
            } else {
                if can_pawn.is_some() && !possible_moves[2..].contains(tile) {
                    continue;
                }
                commands.entity(entity).insert(PossibleMove::Empty);
                *material = tiles_handle.possible_move.clone();
            }
        }
    }
}
