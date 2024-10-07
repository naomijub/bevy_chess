use bevy::{
    color::palettes::css::{DARK_BLUE, GRAY, LIGHT_CYAN, SANDY_BROWN},
    prelude::*,
};

use crate::{board::ActionEvent, ui::models::PlayerMove};

use super::{
    components::{Piece, PieceChoiceUI, PieceColor, PieceType},
    resources::{PieceHandles, PromotionChoice, Sprites},
    setup::{spawn_bishop, spawn_knight, spawn_queen, spawn_rook},
    CanPromoteEvent,
};

const SPEED: f32 = 4.0;

pub fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &mut Piece)>) {
    for (mut transform, mut piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            if piece.first_move {
                piece.first_move = false;
            };
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}

pub fn promote(
    mut event: EventReader<CanPromoteEvent>,
    pieces: Query<(Entity, &Piece)>,
    mut to_promotion: ResMut<PromotionChoice>,
) {
    let Some(promotion) = event.read().next() else {
        return;
    };
    let Ok((entity, piece)) = pieces.get(promotion.0) else {
        return;
    };

    to_promotion.color = Some(piece.color);
    to_promotion.entity = Some(entity);
    to_promotion.piece_type = None;
}

pub fn promotion_ui(
    mut commands: Commands,
    to_promotion: Res<PromotionChoice>,
    ui: Query<Entity, With<PieceChoiceUI>>,
    sprites: Res<Sprites>,
) {
    if to_promotion.can_choose() && ui.is_empty() {
        commands
            .spawn((
                PieceChoiceUI,
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Percent(50.0),
                        min_height: Val::Percent(20.0),
                        position_type: PositionType::Absolute,
                        top: Val::Percent(20.),
                        left: Val::Percent(25.),
                        grid_template_columns: (0..6)
                            .map(|_| GridTrack::auto())
                            .collect::<Vec<_>>(),
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto()],
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                },
            ))
            .with_children(|builder| {
                let row = if to_promotion.color.unwrap() == PieceColor::Black {
                    0usize
                } else {
                    6
                };

                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_column: GridPlacement::start_span(2, 4),
                            padding: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            "Select piece type to promote pawn:",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ));
                    });

                for i in 1..5 {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_column: GridPlacement::start(i as i16 + 1),
                                grid_row: GridPlacement::start(2),
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(50.0),
                                            height: Val::Px(50.0),
                                            border: UiRect::all(Val::Px(3.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::BLACK),
                                        border_radius: BorderRadius::all(Val::Px(3.)),
                                        background_color: SANDY_BROWN.into(),

                                        ..default()
                                    },
                                    match i {
                                        1 => PieceType::Queen,
                                        2 => PieceType::Bishop,
                                        3 => PieceType::Knight,
                                        4 => PieceType::Rook,
                                        _ => PieceType::Pawn,
                                    },
                                ))
                                .with_children(|child| {
                                    let mut text_atlas = TextureAtlas::from(sprites.atlas.clone());
                                    text_atlas.index = row + i;
                                    child.spawn((
                                        ImageBundle {
                                            style: Style {
                                                width: Val::Px(32.),
                                                height: Val::Px(32.),
                                                ..default()
                                            },
                                            image: sprites.image.clone(),
                                            ..default()
                                        },
                                        text_atlas,
                                    ));
                                });
                        });
                }
            });
    } else if to_promotion.is_empty() && !ui.is_empty() {
        let Ok(entity) = ui.get_single() else {
            return;
        };
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &PieceType),
        (Changed<Interaction>, With<Button>),
    >,
    mut to_promotion: ResMut<PromotionChoice>,
) {
    for (interaction, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = DARK_BLUE.into();
                to_promotion.piece_type = Some(*children);
            }
            Interaction::Hovered => {
                border_color.0 = LIGHT_CYAN.into();
            }
            Interaction::None => {
                border_color.0 = GRAY.into();
            }
        }
    }
}

pub fn promote_pawn_to(
    mut to_promotion: ResMut<PromotionChoice>,
    pieces: Query<(Entity, &Piece)>,
    mut commands: Commands,
    handles: Res<PieceHandles>,
    mut action_event: EventWriter<ActionEvent>,
) {
    if to_promotion.is_set() {
        let Ok((entity, piece)) = pieces.get(to_promotion.entity.unwrap()) else {
            return;
        };

        commands.entity(entity).despawn_recursive();
        let color = to_promotion.color.unwrap();
        match to_promotion.piece_type.unwrap() {
            PieceType::King => unreachable!("Can't promote to king"),
            PieceType::Queen => spawn_queen(
                &mut commands,
                if color == PieceColor::Black {
                    handles.black_material.clone()
                } else {
                    handles.white_material.clone()
                },
                color,
                handles.queen_handle.clone(),
                (piece.x, piece.y),
            ),
            PieceType::Bishop => spawn_bishop(
                &mut commands,
                if color == PieceColor::Black {
                    handles.black_material.clone()
                } else {
                    handles.white_material.clone()
                },
                color,
                handles.bishop_handle.clone(),
                (piece.x, piece.y),
            ),
            PieceType::Knight => spawn_knight(
                &mut commands,
                if color == PieceColor::Black {
                    handles.black_material.clone()
                } else {
                    handles.white_material.clone()
                },
                color,
                handles.knight_1_handle.clone(),
                handles.knight_2_handle.clone(),
                (piece.x, piece.y),
            ),
            PieceType::Rook => spawn_rook(
                &mut commands,
                if color == PieceColor::Black {
                    handles.black_material.clone()
                } else {
                    handles.white_material.clone()
                },
                color,
                handles.rook_handle.clone(),
                (piece.x, piece.y),
            ),
            PieceType::Pawn => unreachable!("Can't promote to pawn"),
        };
        action_event.send(ActionEvent {
            action: PlayerMove::promote(piece, &to_promotion.piece_type.unwrap()),
        });
        to_promotion.clear();
    }
}
