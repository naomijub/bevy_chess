use bevy::prelude::*;

use crate::board::ActionEvent;

use super::{
    models::{MovesUI, PlayerMoves},
    MAX_MOVES_LIMIT,
};

pub fn update_moves_ui(mut query: Query<(&mut Text, &MovesUI)>, player_moves: Res<PlayerMoves>) {
    if !player_moves.is_changed() {
        return;
    }
    let moves = player_moves
        .moves
        .iter()
        .enumerate()
        .rev()
        .take(MAX_MOVES_LIMIT)
        .collect::<Vec<_>>();
    for (mut text, MovesUI(index)) in query.iter_mut() {
        if let Some(txt) = text.sections.get_mut(0) {
            txt.value = moves
                .get(*index)
                .map_or(String::new(), |(i, m)| format!("{}: {m}", i + 1));
        }
    }
}

pub fn update_moves_list(
    mut events: EventReader<ActionEvent>,
    mut player_moves: ResMut<PlayerMoves>,
) {
    for event in events.read() {
        player_moves.moves.push(event.action.clone());
    }
}
