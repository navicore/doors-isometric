use bevy::prelude::*;

use crate::state::GameState;

use super::{
    world_component::{CurrentFloorPlan, WorldPlugin},
    world_systems::{handle_floor_plan_event, platform_transition_system, spawn_world},
};

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .add_systems(Update, handle_floor_plan_event)
            .add_systems(Update, platform_transition_system)
            // .add_systems(
            //     Update,
            //     platform_transition_system.run_if(in_state(GameState::TransitioningIn)),
            // )
            .add_systems(OnEnter(GameState::TransitioningOut), spawn_world);
    }
}
