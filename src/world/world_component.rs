#![allow(dead_code)]

use std::time::Duration;

use bevy::prelude::*;

use crate::floorplan::{FloorPlan, Room};

pub struct WorldPlugin;

#[derive(Component, Default)]
pub struct PlatformMarker {}

#[derive(Component, Default)]
pub struct Floor {}

#[derive(Default, Resource, Debug)]
pub struct CurrentFloorPlan {
    pub floorplan: Option<FloorPlan>,
    pub refreshed: Duration, // update every time we sync to the external state
    pub modified: Duration,  // update every time we modify due to changes in the external world
    pub you_are_here: Option<Room>,
    pub previous_room: Option<Room>,
}

#[derive(Component, Default)]
pub struct PlatformTransition {
    pub target_y: f32,
    pub speed: f32,
}

#[derive(Resource, Debug)]
pub struct WorldConfig {
    pub room_x: f32,
    pub room_y: f32,
    pub room_z: f32,
    pub placeholder_y: f32,
    pub exit_room_y: f32,
    pub floor_thickness: f32,
    pub n_rows: usize,
    pub spacing: f32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            room_x: 4.0,
            room_y: 4.0,
            room_z: 4.0,
            placeholder_y: 0.1,
            exit_room_y: 4000.0,
            floor_thickness: 3.0,
            n_rows: 5,
            spacing: 8.0,
        }
    }
}
