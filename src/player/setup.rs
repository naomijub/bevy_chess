use bevy::prelude::*;

#[derive(Component)]
pub struct TurnText;

pub fn add_turn_text(mut commands: Commands) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Player: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 32.0,
                    ..default()
                },
            ),
            TextSection::new(
                "White",
                TextStyle {
                    font_size: 32.0,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(16.),
            left: Val::Px(8.),
            justify_content: JustifyContent::Start,
            ..default()
        }),
        TurnText,
    ));
}
