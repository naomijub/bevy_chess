use bevy::{
    color::palettes::css::{SADDLE_BROWN, SANDY_BROWN},
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
    let mesh = meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(0.98, 0.3, 0.98))));
    let white_material = materials.add(Color::from(SANDY_BROWN));
    let black_material = materials.add(Color::from(SADDLE_BROWN));

    commands.insert_resource(TilesHandles {
        white: white_material.clone(),
        black: black_material.clone(),
        mesh: mesh.clone(),
    });

    // Spawn 64 squares
    for y in 0..8 {
        for x in 0..8 {
            commands.spawn((
                Name::new(format!(
                    "Tile {} [{}, {}]",
                    if (x + y + 1) % 2 == 0 {
                        "white"
                    } else {
                        "black"
                    },
                    x,
                    y
                )),
                PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (x + y + 1) % 2 == 0 {
                        white_material.clone()
                    } else {
                        black_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, -0.15, y as f32)),
                    ..Default::default()
                },
                Square { x, y },
                PickableBundle::default(),
                On::<Pointer<Down>>::send_event::<SelectedEvent>(),
                crate::picking::HIGHLIGHT_TINT.clone(),
            ));
        }
    }
}
