use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use super::{
    player_component::{Action, GroundedState},
    player_systems::{check_grounded, detect_enter_door, player_movement, spawn_player},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GroundedState::default())
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    player_movement.after(check_grounded),
                    check_grounded,
                    detect_enter_door,
                ),
            );
    }
}
