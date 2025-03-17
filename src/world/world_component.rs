#![allow(dead_code)]

use std::time::Duration;

use bevy::prelude::*;

use crate::floorplan::{FloorPlan, Room};

pub struct WorldPlugin;

#[derive(Default, Resource)]
pub struct CurrentFloorPlan {
    pub floorplan: Option<FloorPlan>,
    pub refreshed: Duration, // update every time we sync to the external state
    pub modified: Duration,  // update every time we modify due to changes in the external world
    pub you_are_here: Option<Room>,
    pub you_were_here: Option<Room>,
}
