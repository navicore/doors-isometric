use super::{
    state_component::GameState,
    state_system::{display_paused_text, handle_pause_events, pause_game, remove_pause_text},
};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (pause_game, handle_pause_events))
            .add_systems(OnEnter(GameState::Paused), display_paused_text)
            .add_systems(OnExit(GameState::Paused), remove_pause_text);
    }
}
