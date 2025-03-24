use crate::player::{player_component::GroundedState, Player};

use super::camera_component::MainCamera;
use bevy::prelude::*;

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

// TODO: BUT! jerky movement when the camera follows once the player lands
#[allow(clippy::type_complexity)]
pub fn follow_player(
    time: Res<Time>,
    grounded_state: Res<GroundedState>,
    mut query_set: ParamSet<(
        Query<(&mut Transform, &MainCamera)>,
        Query<&Transform, With<Player>>,
    )>,
) {
    if !grounded_state.0 {
        return; // Stop following if the player is not grounded
    }

    if let Ok(player_transform) = query_set.p1().get_single() {
        let player_position = player_transform.translation;

        for (mut camera_transform, _) in &mut query_set.p0() {
            // Calculate the target position for the camera, adjusting only the z-axis
            let isometric_offset = Vec3::new(-20.0, 20.0, -20.0);
            let target_position = Vec3::new(
                camera_transform.translation.x,         // Keep x steady
                camera_transform.translation.y,         // Keep y steady
                player_position.z + isometric_offset.z, // Adjust z-axis to follow the player
            );

            // Smoothly interpolate the camera's position toward the target position
            let interpolation_speed = 2.0; // Adjust for smoothness
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, interpolation_speed * time.delta_secs());

            // Ensure the camera keeps looking at the player
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }
}
