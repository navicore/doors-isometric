use bevy::prelude::*;
use petgraph::prelude::*;

pub struct WorldPlugin;

#[derive(Component)]
pub struct Room;

#[derive(Component)]
pub struct Door {
    pub from: NodeIndex,
    pub to: NodeIndex,
}
