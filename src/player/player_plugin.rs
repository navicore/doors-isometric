use bevy::prelude::*;

use super::player_systems::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        //.add_systems(Update, player_movement);
    }
}
