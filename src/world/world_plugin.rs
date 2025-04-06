use super::{
    world_component::{CurrentFloorPlan, WorldConfig, WorldPlugin},
    world_systems::{
        handle_floor_plan_event, platform_transition_system, spawn_world, transition_setup,
    },
};
use crate::state::GameState;
use bevy::prelude::*;

fn load_world_config_from_lua() -> WorldConfig {
    use rlua::{Lua, Table};

    let lua = Lua::new();
    std::fs::read_to_string("assets/config.lua").map_or_else(
        |_| WorldConfig::default(),
        |script| {
            lua.load(&script)
                .eval::<Table>()
                .and_then(|config_table| config_table.get::<_, Table>("world_config"))
                .map(|config_table| WorldConfig {
                    room_x: config_table.get("room_x").unwrap_or(4.0),
                    room_y: config_table.get("room_y").unwrap_or(4.0),
                    room_z: config_table.get("room_z").unwrap_or(4.0),
                    placeholder_y: config_table.get("placeholder_y").unwrap_or(0.1),
                    exit_room_y: config_table.get("exit_room_y").unwrap_or(4000.0),
                    floor_thickness: config_table.get("floor_thickness").unwrap_or(3.0),
                    n_rows: config_table.get("n_rows").unwrap_or(5),
                    spacing: config_table.get("spacing").unwrap_or(8.0),
                })
                .unwrap_or_default()
        },
    )
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .insert_resource(load_world_config_from_lua())
            .add_systems(
                Update,
                handle_floor_plan_event.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                transition_setup.run_if(in_state(GameState::TransitioningSetup)),
            )
            .add_systems(OnEnter(GameState::TransitioningIn), spawn_world)
            .add_systems(
                Update,
                platform_transition_system.run_if(in_state(GameState::TransitioningOut)),
            );
    }
}
