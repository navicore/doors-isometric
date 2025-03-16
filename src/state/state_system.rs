use crate::state::GameState;
use bevy::color::Color;
use bevy::prelude::*;
use bevy::text::TextBounds;

use super::state_component::PausedText;

static PAUSED_TEXT_COLOR: Color = Color::srgb(1.0, 0.4, 0.3); // red / orange
                                                              //
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
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => (), //noop
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        // exit the game
        std::process::exit(0);
    }
}

pub fn pause_game(mut time: ResMut<Time<Virtual>>, state: Res<State<GameState>>) {
    if *state == GameState::Paused {
        time.set_relative_speed(0.0); // Freeze physics and animation
    } else {
        time.set_relative_speed(1.0); // Resume physics
    }
}

pub fn display_paused_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    warn!("Displaying paused text"); //ejs
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let box_size = Vec2::new(200.0, 50.0);

    let camera_position = camera_query
        .get_single()
        .map_or(Vec2::ZERO, |camera_transform| {
            camera_transform.translation.truncate()
        });

    let box_position = camera_position + Vec2::new(0.0, 150.0); // Centered relative to camera

    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 35.0,
        ..default()
    };

    commands
        .spawn((
            Sprite::from_color(PAUSED_TEXT_COLOR, box_size),
            Transform::from_translation(box_position.extend(2.0)),
            PausedText,
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("Paused !    "),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary), // Ensure center justification
                TextBounds::from(box_size),
                Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)), // Ensure text is centered in the parent
                PausedText,
            ));
        });
}
