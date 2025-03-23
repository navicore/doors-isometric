use bevy::prelude::Component;
use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Enter,
}

#[derive(Default, Component)]
pub struct Player {
    // pub walk_speed: f32,
    // pub state: PlayerState,
    // pub direction: PlayerDirection,
}
