use std::f32::consts::PI;

use crate::{
    board::components::Square,
    pieces::{
        components::{Piece, PieceColor, PieceType},
        helper::{is_path_empty, possible_moves, Contains},
    },
};
use bevy::{
    color::palettes::css::{GOLD, LIME},
    prelude::*,
};

use super::{setup::TurnText, SelectedPlayerPiece, Turn, VictoryEvent};

pub fn victory_screen(
    mut events: EventReader<VictoryEvent>,
    mut commands: Commands,
    mut turn: ResMut<Turn>,
) {
    let Some(event) = events.read().next() else {
        return;
    };

    let winner = match event.0 {
        PieceColor::White => "White",
        PieceColor::Black => "Black",
    };

    let message = format!("Player {}\n     wins!", winner);

    commands.spawn(
        TextBundle::from_section(
            message,
            TextStyle {
                font_size: 100.,
                color: GOLD.into(),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Vh(35.0),
            left: Val::Vw(30.0),
            justify_content: JustifyContent::Center,
            ..default()
        }),
    );

    *turn = Turn::End;
}

pub fn text_update_system(
    turn: Res<Turn>,
    mut query: Query<&mut Text, With<TurnText>>,
    mut turns_count: Local<u32>,
) {
    if !turn.is_changed() {
        return;
    }
    let next_turn = *turn;
    for mut text in &mut query {
        text.sections[1].value = format!("{next_turn}. Turns: {}", *turns_count);
    }

    *turns_count += 1;
}

pub fn draw_possible_moves(
    mut gizmos: Gizmos,
    selected_piece: Res<SelectedPlayerPiece>,
    pieces: Query<(Entity, &Piece)>,
) {
    let Some(selected_entity) = selected_piece.entity else {
        return;
    };
    let Ok((_, piece)) = pieces.get(selected_entity) else {
        return;
    };
    let begin: Square = piece.into();
    let possible_moves = possible_moves(piece.piece_type, piece.color, &begin, piece.first_move);
    for possible_move in possible_moves
        .into_iter()
        .filter(|end| is_path_empty(&begin, end, &pieces))
        .filter(|pos| {
            pieces
                .color_of(pos)
                .map_or(true, |color| color != piece.color)
        })
    {
        if piece.piece_type == PieceType::Pawn
            && piece.is_diagonal(&possible_move)
            && pieces
                .color_of(&possible_move)
                .map_or(true, |color| color == piece.color)
        {
            continue;
        }
        gizmos.rect(
            Vec3::new(
                possible_move.x as f32 + 0.075,
                0.01,
                possible_move.y as f32 + 0.01,
            ),
            Quat::from_rotation_x(PI / 2.),
            Vec2::splat(0.85),
            LIME,
        );
    }
}
