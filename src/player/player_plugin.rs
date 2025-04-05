use super::{
    player_component::{Action, GroundedState},
    player_systems::{check_grounded, detect_enter_door, player_movement, spawn_player},
};
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GroundedState::default())
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, spawn_player)
            //.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(
                Update,
                (check_grounded, player_movement, detect_enter_door).chain(),
            );
    }
}
