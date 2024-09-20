use bevy::prelude::*;

use super::components::Piece;

pub fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}
