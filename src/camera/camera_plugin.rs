use bevy::prelude::*;

use super::{
    camera_component::IsometricCameraPlugin,
    camera_systems::{follow_player, setup_camera},
};

impl Plugin for IsometricCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, follow_player);
    }
}
