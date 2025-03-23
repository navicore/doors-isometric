use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use super::{
    player_component::Action,
    player_systems::{player_movement, spawn_player},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}
