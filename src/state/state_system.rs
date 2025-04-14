use crate::state::GameState;
use bevy::prelude::*;

use super::state_component::{GameOverReason, PausedText, QuitTimer};

pub fn remove_pause_text(mut commands: Commands, query: Query<Entity, With<PausedText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_pause_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => {
                // Pause the game
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                // Resume the game
                next_state.set(GameState::InGame);
            }
            _ => (),
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        if *state != GameState::InGame {
            // Quit the game immediately - they already are in a paused state
            std::process::exit(0);
        }
        next_state.set(GameState::GameOver {
            reason: GameOverReason::PlayerQuit,
        });
    }
}

pub fn pause_game(mut time: ResMut<Time<Virtual>>, state: Res<State<GameState>>) {
    if *state == GameState::Paused {
        time.set_relative_speed(0.0); // Freeze physics and animation
    } else {
        time.set_relative_speed(1.0); // Resume physics
    }
}

pub fn display_paused_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("game\npaused"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 67.0,
            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        PausedText,
    ));
}

pub fn display_game_over_player_fell_text(commands: Commands, asset_server: Res<AssetServer>) {
    display_game_over_text(commands, asset_server, "Player Fell");
}

pub fn display_game_over_player_quit_text(commands: Commands, asset_server: Res<AssetServer>) {
    display_game_over_text(commands, asset_server, "Player Quit");
}

pub fn setup_quit_timer(mut commands: Commands) {
    commands.insert_resource(QuitTimer(Timer::from_seconds(0.75, TimerMode::Once)));
}

pub fn run_quit_after_delay(time: Res<Time>, mut timer: ResMut<QuitTimer>) {
    if timer.0.tick(time.delta()).finished() {
        std::process::exit(0);
    }
}

fn display_game_over_text(mut commands: Commands, asset_server: Res<AssetServer>, reason: &str) {
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new(format!("Game\nOver\n{reason}")),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 67.0,
            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        PausedText,
    ));
}
