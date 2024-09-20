use bevy::prelude::*;

use crate::{pieces::components::Piece, player::SelectedPlayerPiece};

use super::{components::Square, MoveToEvent, SelectedEvent, SelectedSquare};

pub fn set_move_to_square(
    mut move_to_event: EventReader<MoveToEvent>,
    mut pieces: Query<&mut Piece>,
) {
    for event in move_to_event.read() {
        if let Ok(mut piece) = pieces.get_mut(event.entity) {
            piece.x = event.to.0 as u8;
            piece.y = event.to.1 as u8;
        }
    }
    move_to_event.clear();
}

pub fn set_selections(
    mut events: EventReader<SelectedEvent>,
    mut move_to_event: EventWriter<MoveToEvent>,
    mut selected_sq: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPlayerPiece>,
    mouse_button_inputs: Res<ButtonInput<MouseButton>>,
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
                if piece.is_move_valid(square, &pieces) {
                    move_to_event.send(MoveToEvent {
                        entity: piece_entity,
                        to: (square.x, square.y),
                    });
                }
            }
            selected_piece.entity = None;
            selected_piece.entity = None;
        } else {
            for (piece_entity, piece) in pieces.iter() {
                if piece.x == square.x as u8 && piece.y == square.y as u8 {
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
