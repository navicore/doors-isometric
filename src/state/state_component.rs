#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    TransitioningOut,
    TransitioningIn,
}

#[derive(Debug, Component)]
pub struct PausedText;
