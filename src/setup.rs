use bevy::{prelude::*, render::camera::ScalingMode};

pub fn ambient(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::FixedVertical(10.0),
            ..default()
        }
        .into(),
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..default()
    });
    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    ));
}
