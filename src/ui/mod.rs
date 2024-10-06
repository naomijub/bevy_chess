use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use bevy_mod_picking::prelude::Pickable;
use models::{MovesUI, PlayerMoves};
use systems::{update_moves_list, update_moves_ui};

pub mod helpers;
pub mod models;
pub mod systems;

const FONT_SIZE: f32 = 20.;
const TITLE_FONT_SIZE: f32 = 28.;
const LINE_HEIGHT: f32 = 21.;
pub const MAX_MOVES_LIMIT: usize = 20;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMoves>()
            .add_systems(Startup, setup)
            .add_systems(Update, (update_moves_ui, update_moves_list));
    }
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                min_width: Val::Px(400.0),
                width: Val::Px(400.),
                height: Val::Percent(90.0),
                top: Val::Px(60.),
                right: Val::Px(32.),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        })
        .insert(Pickable::IGNORE)
        .with_children(|parent| {
            // container for all other examples
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // vertical scroll example
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Title
                            parent.spawn((
                                TextBundle::from_section(
                                    "Last moves history",
                                    TextStyle {
                                        font_size: TITLE_FONT_SIZE,
                                        ..default()
                                    },
                                ),
                                Label,
                            ));
                            // Scrolling list
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_self: AlignSelf::Stretch,
                                        height: Val::Percent(50.),
                                        overflow: Overflow::clip_y(), // scroll_y(), // n.b.
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.10, 0.10, 0.10).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..MAX_MOVES_LIMIT {
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    min_height: Val::Px(LINE_HEIGHT),
                                                    max_height: Val::Px(LINE_HEIGHT + 2.0),
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .insert(Pickable {
                                                should_block_lower: false,
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent
                                                    .spawn((
                                                        TextBundle::from_section(
                                                            format!("{i}"),
                                                            TextStyle {
                                                                font_size: FONT_SIZE,
                                                                ..default()
                                                            },
                                                        ),
                                                        Label,
                                                        MovesUI(i),
                                                        AccessibilityNode(NodeBuilder::new(
                                                            Role::ListItem,
                                                        )),
                                                    ))
                                                    .insert(Pickable {
                                                        should_block_lower: false,
                                                        ..default()
                                                    });
                                            });
                                    }
                                });
                        });
                });
        });
}
