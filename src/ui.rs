use bevy::{a11y::{accesskit::{NodeBuilder, Role}, AccessibilityNode},  prelude::*};
use bevy_mod_picking::prelude::Pickable;

const FONT_SIZE: f32 = 20.;
const LINE_HEIGHT: f32 = 21.;
const MAX_MOVES_LIMIT: usize = 20;
pub struct UiPlugin;

impl Plugin for UiPlugin {  
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMoves>()
            .add_systems(Startup, setup);

    }
}

#[derive(Debug, Component, Clone, Reflect, Default)]
pub struct MovesUI;

#[derive(Debug, Resource, Clone, Reflect, Default)]
pub struct PlayerMoves {
    pub moves: Vec<String>
}

pub fn setup(mut commands: Commands) {
    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Px(80.0),
            height: Val::Percent(80.0),
            top: Val::Px(60.),
            left: Val::Px(16.),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
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
                        width: Val::Px(200.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Vertically Scrolling List",
                            TextStyle {
                                font_size: FONT_SIZE,
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
                            for i in 0..25 {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            min_height: Val::Px(LINE_HEIGHT),
                                            max_height: Val::Px(LINE_HEIGHT),
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
                                                    format!("Item {i}"),
                                                    TextStyle {
                                                        ..default()
                                                    },
                                                ),
                                                Label,
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
