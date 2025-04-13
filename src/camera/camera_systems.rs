use crate::player::Player;

use super::camera_component::{CameraAction, MainCamera};
use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, ButtonlikeChord, InputMap},
    InputManagerBundle,
};

const SPEED: f32 = 0.01;

pub fn move_camera(
    mut query: Query<(&ActionState<CameraAction>, &mut Transform), With<MainCamera>>,
) {
    if let Ok((action_state, mut transform)) = query.get_single_mut() {
        if action_state.pressed(&CameraAction::TranslationXInc) {
            let t = transform.rotation * Vec3::X * SPEED * 10.0;
            transform.translation += t;
        }
        if action_state.pressed(&CameraAction::TranslationXDec) {
            let t = transform.rotation * Vec3::X * SPEED * 10.0;
            transform.translation -= t;
        }

        if action_state.pressed(&CameraAction::TranslationYInc) {
            let t = transform.rotation * Vec3::Y * SPEED * 10.0;
            transform.translation += t;
        }
        if action_state.pressed(&CameraAction::TranslationYDec) {
            let t = transform.rotation * Vec3::Y * SPEED * 10.0;
            transform.translation -= t;
        }

        if action_state.pressed(&CameraAction::TranslationZInc) {
            let t = transform.rotation * Vec3::Z * SPEED * 10.0;
            transform.translation += t;
        }
        if action_state.pressed(&CameraAction::TranslationZDec) {
            let t = transform.rotation * Vec3::Z * SPEED * 10.0;
            transform.translation -= t;
        }

        if action_state.pressed(&CameraAction::RotateXInc) {
            transform.rotation *= Quat::from_rotation_x(SPEED);
        }
        if action_state.pressed(&CameraAction::RotateXDec) {
            transform.rotation *= Quat::from_rotation_x(-SPEED);
        }

        if action_state.pressed(&CameraAction::RotateYInc) {
            transform.rotation *= Quat::from_rotation_y(SPEED);
        }
        if action_state.pressed(&CameraAction::RotateYDec) {
            transform.rotation *= Quat::from_rotation_y(-SPEED);
        }

        if action_state.pressed(&CameraAction::RotateZInc) {
            transform.rotation *= Quat::from_rotation_z(SPEED);
        }
        if action_state.pressed(&CameraAction::RotateZDec) {
            transform.rotation *= Quat::from_rotation_z(-SPEED);
        }
        if action_state.pressed(&CameraAction::ResetXYZ) {
            *transform = default_camera_transform();
        }
    }
}

fn default_camera_transform() -> Transform {
    // Standard isometric angles: rotated 45° horizontally, ~35.26° vertically
    let translation = Vec3::new(-20.0, 20.0, -20.0);
    let target = Vec3::ZERO;
    Transform::from_translation(translation).looking_at(target, Vec3::Y)
}

pub fn setup_camera(mut commands: Commands) {
    let input_map = InputMap::new([
        (
            CameraAction::TranslationXInc,
            ButtonlikeChord::new([KeyCode::KeyX, KeyCode::ArrowUp]),
        ),
        (
            CameraAction::TranslationXDec,
            ButtonlikeChord::new([KeyCode::KeyX, KeyCode::ArrowDown]),
        ),
        (
            CameraAction::TranslationYInc,
            ButtonlikeChord::new([KeyCode::KeyY, KeyCode::ArrowUp]),
        ),
        (
            CameraAction::TranslationYDec,
            ButtonlikeChord::new([KeyCode::KeyY, KeyCode::ArrowDown]),
        ),
        (
            CameraAction::TranslationZInc,
            ButtonlikeChord::new([KeyCode::KeyZ, KeyCode::ArrowUp]),
        ),
        (
            CameraAction::TranslationZDec,
            ButtonlikeChord::new([KeyCode::KeyZ, KeyCode::ArrowDown]),
        ),
        (
            CameraAction::RotateXInc,
            ButtonlikeChord::new([KeyCode::KeyX, KeyCode::ArrowRight]),
        ),
        (
            CameraAction::RotateXDec,
            ButtonlikeChord::new([KeyCode::KeyX, KeyCode::ArrowLeft]),
        ),
        (
            CameraAction::RotateYInc,
            ButtonlikeChord::new([KeyCode::KeyY, KeyCode::ArrowRight]),
        ),
        (
            CameraAction::RotateYDec,
            ButtonlikeChord::new([KeyCode::KeyY, KeyCode::ArrowLeft]),
        ),
        (
            CameraAction::RotateZInc,
            ButtonlikeChord::new([KeyCode::KeyZ, KeyCode::ArrowRight]),
        ),
        (
            CameraAction::RotateZDec,
            ButtonlikeChord::new([KeyCode::KeyZ, KeyCode::ArrowLeft]),
        ),
        (
            CameraAction::ResetXYZ,
            ButtonlikeChord::new([KeyCode::KeyX, KeyCode::KeyY, KeyCode::KeyZ]),
        ),
    ]);

    commands.spawn((
        Camera3d::default(),
        MainCamera,
        default_camera_transform(),
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
            rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4)
                * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
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
