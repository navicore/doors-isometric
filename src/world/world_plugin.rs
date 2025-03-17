use bevy::prelude::*;

use super::{
    world_component::{CurrentFloorPlan, WorldPlugin},
    world_systems::handle_floor_plan_event,
};

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .add_systems(Update, handle_floor_plan_event);
    }
}
