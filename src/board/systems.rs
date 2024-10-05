use bevy::prelude::*;

use crate::{
    pieces::components::Piece,
    player::{SelectedPlayerPiece, Turn, VictoryEvent},
};

use super::{components::Square, DespawnEvent, MoveToEvent, SelectedEvent, SelectedSquare};

pub fn set_move_to_square(
    mut move_to_event: EventReader<MoveToEvent>,
    mut pieces: Query<&mut Piece>,
    mut turn: ResMut<Turn>,
) {
    for event in move_to_event.read() {
        if let Ok(mut piece) = pieces.get_mut(event.entity) {
            piece.x = event.to.0;
            piece.y = event.to.1;
            *turn = piece.color.opposite().into();
            piece.first_move = false;
        }
    }
    move_to_event.clear();
}

pub fn despawn_taken(mut despawn_event: EventReader<DespawnEvent>, mut commands: Commands) {
    if let Some(event) = despawn_event.read().next() {
        commands.entity(event.0).despawn_recursive();
    }

    despawn_event.clear();
}

pub fn set_selections(
    mut events: EventReader<SelectedEvent>,
    mut move_to_event: EventWriter<MoveToEvent>,
    despawn_event: EventWriter<DespawnEvent>,
    victory_event: EventWriter<VictoryEvent>,
    mut selected_sq: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPlayerPiece>,
    mouse_button_inputs: Res<ButtonInput<MouseButton>>,
    turn: Res<Turn>,
    pieces: Query<(Entity, &Piece)>,
    tiles: Query<(Entity, &Square)>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(&SelectedEvent(selected_sq_entity)) = events.read().next() {
        selected_sq.entity = Some(selected_sq_entity);
        let Ok((_, square)) = tiles.get(selected_sq_entity) else {
            return;
        };

        if let Some(selected_piece_entity) = selected_piece.entity {
            if let Ok((piece_entity, piece)) = pieces.get(selected_piece_entity) {
                let valid_move = piece.is_move_valid(square, &pieces);
                if valid_move.0 {
                    move_to_event.send(MoveToEvent {
                        entity: piece_entity,
                        to: (square.x, square.y),
                    });
                    // Rook castle move
                    if let Some((entity, column)) = valid_move.1 {
                        move_to_event.send(MoveToEvent {
                            entity,
                            to: (square.x, column),
                        });
                    }

                    kill_piece(&pieces, square, piece, victory_event, despawn_event);
                }
            }
            selected_piece.entity = None;
            selected_piece.entity = None;
        } else {
            for (piece_entity, piece) in pieces.iter() {
                if piece.x == square.x && piece.y == square.y && *turn == piece {
                    selected_piece.entity = Some(piece_entity);
                    break;
                }
            }
        }
    } else {
        selected_sq.entity = None;
        selected_piece.entity = None;
    }

    events.clear();
}

fn kill_piece(
    pieces: &Query<'_, '_, (Entity, &Piece)>,
    square: &Square,
    piece: &Piece,
    mut victory_event: EventWriter<'_, VictoryEvent>,
    mut despawn_event: EventWriter<'_, DespawnEvent>,
) {
    if let Some((entity, other_piece)) = pieces
        .iter()
        .find(|p| p.1.x == square.x && p.1.y == square.y && p.1.color != piece.color)
    {
        if other_piece.is_king() {
            victory_event.send(VictoryEvent(piece.color));
        }
        despawn_event.send(DespawnEvent(entity));
    }
}
