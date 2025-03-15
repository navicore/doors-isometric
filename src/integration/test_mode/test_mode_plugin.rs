use crate::cli;
use bevy::prelude::*;
use clap::Parser;

use super::test_mode_systems::{
    fire_room25_floorplan_event, fire_room2_floorplan_event, fire_room5_floorplan_event,
};

pub struct TestModeIntegrationPlugin;

impl Plugin for TestModeIntegrationPlugin {
    #[allow(clippy::branches_sharing_code)]
    fn build(&self, app: &mut App) {
        let room_generator = cli::Cli::parse().room_generator;
        add_room_generator_system(app, room_generator);
    }
}

fn add_room_generator_system(app: &mut App, room_generator: Option<cli::RoomGeneratorType>) {
    match room_generator {
        Some(cli::RoomGeneratorType::Rooms2) => {
            app.add_systems(Startup, fire_room2_floorplan_event);
        }
        Some(cli::RoomGeneratorType::Rooms25) => {
            app.add_systems(Startup, fire_room25_floorplan_event);
        }
        Some(cli::RoomGeneratorType::Rooms5) => {
            app.add_systems(Startup, fire_room5_floorplan_event);
        }
        _ => {
            panic!("Invalid room generator type");
        }
    }
}
