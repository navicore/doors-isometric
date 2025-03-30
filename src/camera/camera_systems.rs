use crate::player::Player;

use super::camera_component::{CameraAction, MainCamera};
use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

pub fn move_camera(
    mut query: Query<(&ActionState<CameraAction>, &mut Transform), With<MainCamera>>,
) {
    if let Ok((action_state, mut transform)) = query.get_single_mut() {
        let rotation_speed = 0.01; // Adjust rotation speed as needed

        if action_state.pressed(&CameraAction::PanLeft) {
            transform.rotation *= Quat::from_rotation_y(rotation_speed);
            transform.rotation *= Quat::from_rotation_z(rotation_speed);
        }
        if action_state.pressed(&CameraAction::PanRight) {
            transform.rotation *= Quat::from_rotation_y(-rotation_speed);
            transform.rotation *= Quat::from_rotation_z(-rotation_speed);
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    // Standard isometric angles: rotated 45° horizontally, ~35.26° vertically
    let translation = Vec3::new(-20.0, 20.0, -20.0);
    let target = Vec3::ZERO;
    let camera_transform = Transform::from_translation(translation).looking_at(target, Vec3::Y);

    let input_map = InputMap::new([
        (CameraAction::PanLeft, KeyCode::F8),
        (CameraAction::PanRight, KeyCode::F9),
    ]);

    commands.spawn((
        Camera3d::default(),
        MainCamera,
        camera_transform,
        InputManagerBundle::with_map(input_map),
    ));

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

    if let Ok(mut camera_transform) = param_set.p1().get_single_mut() {
        camera_transform.translation.z = player_position.z - 10.0;
        // todo it would be better to only move the camera as the player approached the edge of the
        // screen and then smoothly recenter
    }
}
