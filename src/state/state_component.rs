#![allow(dead_code)]

use std::fmt::{Display, Formatter};

use bevy::prelude::*;

#[derive(Resource)]
pub struct QuitTimer(pub Timer);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    TransitioningOutSetup,
    TransitioningOut,
    TransitioningInSetup,
    TransitioningIn,
    TransitioningComplete,
    GameOver {
        reason: GameOverReason,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameOverReason {
    PlayerFell,
    PlayerQuit,
}

impl Display for GameOverReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayerFell => write!(f, "Player fell"),
            Self::PlayerQuit => write!(f, "Player quit"),
        }
    }
}

#[derive(Debug, Component)]
pub struct PausedText;
