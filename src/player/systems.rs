use crate::pieces::components::PieceColor;
use bevy::{color::palettes::css::GOLD, prelude::*};

use super::{Turn, VictoryEvent};

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
