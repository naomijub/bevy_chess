use bevy::{
    color::palettes::css::{SADDLE_BROWN, SANDY_BROWN},
    prelude::*,
};
use bevy_mod_picking::PickableBundle;
use components::Square;

pub mod components;

pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(Plane3d::default().mesh().size(1., 1.)));
    let white_material = materials.add(Color::from(SANDY_BROWN));
    let black_material = materials.add(Color::from(SADDLE_BROWN));

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
                crate::picking::HIGHLIGHT_TINT.clone(),
            ));
        }
    }
}
