use crate::state::GameState;

use super::{
    player_component::{Action, GroundedState, PlayerConfig, PlayerStartPosition},
    player_systems::{
        detect_enter_door, detect_grounded, detect_wall_collision, player_movement, spawn_player,
    },
};
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn load_player_config_from_lua() -> PlayerConfig {
    use rlua::{Lua, Table};

    let lua = Lua::new();
    std::fs::read_to_string("assets/config.lua").map_or_else(
        |_| PlayerConfig::default(),
        |script| {
            lua.load(&script)
                .eval::<Table>()
                .and_then(|config_table| config_table.get::<_, Table>("player_config"))
                .map(|config_table| PlayerConfig {
                    x: config_table.get("x").unwrap_or(1.2),
                    y: config_table.get("y").unwrap_or(1.2),
                    z: config_table.get("z").unwrap_or(1.2),
                    jump_force: config_table.get("jump_force").unwrap_or(3.0),
                    gravity_scale: config_table.get("gravity_scale").unwrap_or(2.5),
                    mass: config_table.get("mass").unwrap_or(4.0),
                    dynamic_coefficient: config_table.get("dynamic_coefficient").unwrap_or(0.3),
                    static_coefficient: config_table.get("static_coefficient").unwrap_or(0.5),
                })
                .unwrap_or_default()
        },
    )
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GroundedState::default())
            .insert_resource(PlayerStartPosition::default())
            .insert_resource(load_player_config_from_lua())
            .add_plugins(InputManagerPlugin::<Action>::default())
            //.add_systems(Startup, spawn_player)
            .add_systems(OnEnter(GameState::TransitioningComplete), spawn_player)
            .add_systems(
                Update,
                (
                    detect_grounded,
                    player_movement,
                    detect_enter_door,
                    detect_wall_collision,
                )
                    .chain(),
            );
    }
}
