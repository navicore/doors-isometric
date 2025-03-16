use bevy::prelude::*;

use super::camera_component::MainCamera;

pub fn setup_camera(mut commands: Commands) {
    // Standard isometric angles: rotated 45° horizontally, ~35.26° vertically
    let translation = Vec3::new(-20.0, 20.0, -20.0);
    let target = Vec3::ZERO;
    let camera_transform = Transform::from_translation(translation).looking_at(target, Vec3::Y);

    commands.spawn((Camera3d::default(), MainCamera, camera_transform));

    // Add a simple directional light for basic illumination
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(-10.0, 15.0, -10.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
    ));
}
