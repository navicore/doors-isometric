use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use super::{
    camera_component::{CameraAction, IsometricCameraPlugin},
    camera_systems::{follow_player, move_camera, setup_camera},
};

impl Plugin for IsometricCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (follow_player, move_camera));
    }
}
