use super::components::{Piece, PieceColor, PieceType};
use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};
use bevy_mod_picking::prelude::Pickable;

static GLB_PIECES_PATH: &str = "models/pieces.glb";

pub fn add_pieces_name(mut commands: Commands, pieces: Query<(Entity, &Piece)>) {
    for (entity, piece) in pieces.iter() {
        commands.entity(entity).insert(piece.name());
    }
}

pub fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh0/Primitive0", GLB_PIECES_PATH));
    let king_cross_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh1/Primitive0", GLB_PIECES_PATH));
    let pawn_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh2/Primitive0", GLB_PIECES_PATH));
    let knight_1_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh3/Primitive0", GLB_PIECES_PATH));
    let knight_2_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh4/Primitive0", GLB_PIECES_PATH));
    let rook_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh5/Primitive0", GLB_PIECES_PATH));
    let bishop_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh6/Primitive0", GLB_PIECES_PATH));
    let queen_handle: Handle<Mesh> =
        asset_server.load(format!("{}#Mesh7/Primitive0", GLB_PIECES_PATH));

    let black_material = materials.add(Color::from(BLACK));
    let white_material = materials.add(Color::from(WHITE));

    spawn_rook(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 0),
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 1),
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 2),
    );
    spawn_queen(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        queen_handle.clone(),
        (0, 3),
    );
    spawn_king(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        king_handle.clone(),
        king_cross_handle.clone(),
        (0, 4),
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 5),
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 6),
    );
    spawn_rook(
        &mut commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            pawn_handle.clone(),
            (1, i as i8),
        );
    }

    spawn_rook(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 0),
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (7, 1),
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        bishop_handle.clone(),
        (7, 2),
    );
    spawn_queen(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        queen_handle,
        (7, 3),
    );
    spawn_king(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        king_handle,
        king_cross_handle,
        (7, 4),
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        bishop_handle,
        (7, 5),
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        knight_1_handle,
        knight_2_handle,
        (7, 6),
    );
    spawn_rook(
        &mut commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle,
        (7, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            pawn_handle.clone(),
            (6, i as i8),
        );
    }
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (i8, i8),
) {
    commands
        // Spawn parent entity
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            Piece {
                color: piece_color,
                piece_type: PieceType::King,
                x: position.0,
                y: position.1,
                first_move: true,
            },
            Pickable::IGNORE,
        ))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: mesh_cross,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
        });
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: (i8, i8),
) {
    commands
        // Spawn parent entity
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            Piece {
                color: piece_color,
                piece_type: PieceType::Knight,
                x: position.0,
                y: position.1,
                first_move: true,
            },
            Pickable::IGNORE,
        ))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: mesh_1,
                    material: material.clone(),
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: mesh_2,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
        });
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (i8, i8),
) {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            Piece {
                color: piece_color,
                piece_type: PieceType::Queen,
                x: position.0,
                y: position.1,
                first_move: true,
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
        });
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (i8, i8),
) {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            Piece {
                color: piece_color,
                piece_type: PieceType::Bishop,
                x: position.0,
                y: position.1,
                first_move: true,
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
        });
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (i8, i8),
) {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            Piece {
                color: piece_color,
                piece_type: PieceType::Rook,
                x: position.0,
                y: position.1,
                first_move: true,
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
        });
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (i8, i8),
) {
    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
                ..Default::default()
            },
            Piece {
                color: piece_color,
                piece_type: PieceType::Pawn,
                x: position.0,
                y: position.1,
                first_move: true,
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                        transform.scale *= Vec3::splat(0.2);
                        transform
                    },
                    ..Default::default()
                },
                Pickable::IGNORE,
            ));
        });
}
