use crate::player::Player;

use super::camera_component::MainCamera;
use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

const CAMERA_MOVE_SPEED: f32 = 100.0; // Speed at which the camera moves
const SCREEN_HALF_WIDTH: f32 = WINDOW_WIDTH / 2.0; // Half of window width (assuming 1200x800 resolution)
const SCREEN_HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.0; // Half of window width (assuming 1200x800 resolution)
const SCROLL_THRESHOLD_XY: f32 = 400.0; // Distance from the screen edge before scrolling
const SCROLL_THRESHOLD_Z: f32 = 200.0; // Distance from the screen edge before scrolling

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

#[allow(clippy::type_complexity)]
pub fn follow_player(
    mut param_set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
) {
    let player_position = if let Ok(player_transform) = param_set.p0().get_single() {
        player_transform.translation
    } else {
        return;
    };

    // TODO debug this - camera is not moving except after the player falls off the end then it
    // lurches forward in steps that eventually overshoot the platform.
    if let Ok(mut camera_transform) = param_set.p1().get_single_mut() {
        let camera_x = camera_transform.translation.x;
        let camera_z = camera_transform.translation.z;

        // If player is near the right edge of the screen
        if player_position.x > camera_x + SCREEN_HALF_WIDTH - SCROLL_THRESHOLD_XY {
            camera_transform.translation.x += CAMERA_MOVE_SPEED;
        }
        // If player is near the left edge of the screen
        if player_position.x < camera_x - SCREEN_HALF_WIDTH + SCROLL_THRESHOLD_XY {
            camera_transform.translation.x -= CAMERA_MOVE_SPEED;
        }

        // If player is near the top edge of the screen
        if player_position.z > camera_z + SCREEN_HALF_HEIGHT - SCROLL_THRESHOLD_Z {
            camera_transform.translation.z += CAMERA_MOVE_SPEED;
        }
        // If player is near the bottom edge of the screen
        if player_position.z < camera_z - SCREEN_HALF_HEIGHT + SCROLL_THRESHOLD_Z {
            camera_transform.translation.z -= CAMERA_MOVE_SPEED;
        }
    }
}
