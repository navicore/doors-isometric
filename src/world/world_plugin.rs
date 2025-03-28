use bevy::prelude::*;

use crate::state::GameState;

use super::{
    world_component::{CurrentFloorPlan, WorldPlugin},
    world_systems::{handle_floor_plan_event, spawn_world},
};

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .add_systems(Update, handle_floor_plan_event)
            .add_systems(
                OnEnter(GameState::Transitioning),
                spawn_world.after(handle_floor_plan_event),
            );
    }
}
