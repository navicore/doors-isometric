#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Welcome,
    InGame,
    Paused,
    TransitioningIn,
    TransitioningOut,
}

#[derive(Debug, Component)]
pub struct PausedText;
