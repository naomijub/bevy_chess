use bevy::{
    color::palettes::{
        css::{SADDLE_BROWN, SANDY_BROWN},
        tailwind::{BLUE_500, GREEN_100, RED_300},
    },
    prelude::*,
};
use bevy_mod_picking::prelude::*;
use components::Square;
use systems::{define_possible_moves, set_selected_piece};

pub mod components;
pub mod systems;

pub fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(Plane3d::default().mesh().size(1., 1.)));
    let white_material = materials.add(Color::from(SANDY_BROWN));
    let black_material = materials.add(Color::from(SADDLE_BROWN));
    let possible_move = materials.add(Color::from(GREEN_100));
    let blocked_move = materials.add(Color::from(RED_300));
    let enemy_piece = materials.add(Color::from(BLUE_500));

    commands.insert_resource(TilesHandles {
        white: white_material.clone(),
        black: black_material.clone(),
        possible_move,
        blocked_move,
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
                crate::picking::HIGHLIGHT_TINT.clone(),
                On::<Pointer<Click>>::send_event::<SelectedEvent>(),
            ));
        }
    }
}

#[derive(Debug, Clone, Reflect, Event)]
pub struct SelectedEvent(Entity);

#[derive(Debug, Clone, Reflect, Resource)]
pub struct TilesHandles {
    pub white: Handle<StandardMaterial>,
    pub black: Handle<StandardMaterial>,
    pub possible_move: Handle<StandardMaterial>,
    pub blocked_move: Handle<StandardMaterial>,
    pub enemy_piece: Handle<StandardMaterial>,
    pub mesh: Handle<Mesh>,
}

impl From<ListenerInput<Pointer<Click>>> for SelectedEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target)
    }
}

#[derive(Resource, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectedEvent>()
            .add_systems(Startup, setup_board)
            .add_systems(
                Update,
                (
                    set_selected_piece.before(define_possible_moves),
                    define_possible_moves,
                ),
            );
    }
}
