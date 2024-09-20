use bevy::{
    color::palettes::{
        css::{SADDLE_BROWN, SANDY_BROWN},
        tailwind::{BLUE_500, GREEN_300},
    },
    prelude::*,
};
use bevy_mod_picking::prelude::*;

use super::{components::Square, SelectedEvent, TilesHandles};

pub fn board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(Plane3d::default().mesh().size(0.98, 0.98)));
    let white_material = materials.add(Color::from(SANDY_BROWN));
    let black_material = materials.add(Color::from(SADDLE_BROWN));
    let possible_move = materials.add(Color::from(GREEN_300));
    let enemy_piece = materials.add(Color::from(BLUE_500));

    commands.insert_resource(TilesHandles {
        white: white_material.clone(),
        black: black_material.clone(),
        possible_move,
        enemy_piece,
        mesh: mesh.clone(),
    });

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn((
                Name::new(format!(
                    "Tile {} [{}, {}]",
                    if (i + j + 1) % 2 == 0 {
                        "white"
                    } else {
                        "black"
                    },
                    i,
                    j
                )),
                PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        white_material.clone()
                    } else {
                        black_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                },
                Square { x: i, y: j },
                PickableBundle::default(),
                On::<Pointer<Down>>::send_event::<SelectedEvent>(),
                crate::picking::HIGHLIGHT_TINT.clone(),
            ));
        }
    }
}
