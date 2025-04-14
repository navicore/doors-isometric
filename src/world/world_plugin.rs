use super::{
    world_component::{
        CurrentFloorPlan, DisplayRoomInfoEvent, NextFloorPlan, WorldConfig, WorldPlugin,
    },
    world_systems::{
        display_room_info_text, handle_floor_plan_event, platform_transition_in,
        platform_transition_in_setup, platform_transition_out, platform_transition_out_setup,
        remove_room_info_text, setup_quit_displaying_room_info_text_timer, update_wall_state,
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
                    n_columns: config_table.get("n_columns").unwrap_or(5),
                    spacing: config_table.get("spacing").unwrap_or(8.0),
                })
                .unwrap_or_default()
        },
    )
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .add_event::<DisplayRoomInfoEvent>()
            .insert_resource(NextFloorPlan::default())
            .insert_resource(load_world_config_from_lua())
            .add_systems(
                Update,
                (
                    handle_floor_plan_event,
                    display_room_info_text,
                    remove_room_info_text,
                    setup_quit_displaying_room_info_text_timer,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    platform_transition_out_setup
                        .run_if(in_state(GameState::TransitioningOutSetup)),
                    platform_transition_out.run_if(in_state(GameState::TransitioningOut)),
                    platform_transition_in_setup.run_if(in_state(GameState::TransitioningInSetup)),
                    platform_transition_in.run_if(in_state(GameState::TransitioningIn)),
                    update_wall_state,
                ),
            );
    }
}
