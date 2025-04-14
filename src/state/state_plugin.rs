use super::{
    state_component::{GameOverReason, GameState},
    state_system::{
        display_game_over_player_fell_text, display_game_over_player_quit_text,
        display_paused_text, handle_pause_events, pause_game, remove_pause_text,
        run_quit_after_delay, setup_quit_timer,
    },
};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (pause_game, handle_pause_events))
            .add_systems(OnEnter(GameState::Paused), display_paused_text)
            .add_systems(
                OnEnter(GameState::GameOver {
                    reason: GameOverReason::PlayerQuit,
                }),
                (display_game_over_player_quit_text, setup_quit_timer),
            )
            .add_systems(
                Update,
                run_quit_after_delay.run_if(in_state(GameState::GameOver {
                    reason: GameOverReason::PlayerQuit,
                })),
            )
            .add_systems(
                OnEnter(GameState::GameOver {
                    reason: GameOverReason::PlayerFell,
                }),
                display_game_over_player_fell_text,
            )
            .add_systems(OnExit(GameState::Paused), remove_pause_text);
    }
}
