use super::player_component::{
    Action, Grounded, Player, PlayerBundle, PlayerDirection, PlayerState, PLAYER_SHAPE_X,
    PLAYER_SHAPE_Y, PLAYER_SHAPE_Z,
};
use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*};
use leafwing_input_manager::prelude::ActionState;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO integrate these in bundle so they can change
    let player_shape = meshes.add(Cuboid::new(PLAYER_SHAPE_X, PLAYER_SHAPE_Y, PLAYER_SHAPE_Z)); // Player size
    let player_material = materials.add(Color::from(BLUE_600));

    commands.spawn((
        Mesh3d(player_shape),
        MeshMaterial3d(player_material),
        PlayerBundle::new(),
    ));
}

pub fn player_movement(
    mut query: Query<
        (
            &mut ExternalForce,
            &Grounded,
            &ActionState<Action>,
            &mut Player,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut force, grounded, action_state, mut player)) = query.get_single_mut() {
        if !grounded.0 {
            debug!("Player is not grounded");
        }

        let mut direction = Vec3::ZERO;

        let mut pressed = if action_state.pressed(&Action::MoveForward) {
            direction.z += 1.0;
            player.direction = PlayerDirection::Up;
            player.state = PlayerState::Walk;
            true
        } else {
            false
        };

        pressed = if action_state.pressed(&Action::MoveBackward) {
            direction.z -= 1.0;
            player.direction = PlayerDirection::Down;
            player.state = PlayerState::Walk;
            true
        } else {
            pressed
        };

        pressed = if action_state.pressed(&Action::MoveLeft) {
            direction.x += 1.0;
            player.direction = PlayerDirection::Left;
            player.state = PlayerState::Walk;
            true
        } else {
            pressed
        };

        pressed = if action_state.pressed(&Action::MoveRight) {
            direction.x -= 1.0;
            player.direction = PlayerDirection::Right;
            player.state = PlayerState::Walk;
            true
        } else {
            pressed
        };

        // Normalize direction and apply force
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        force.apply_force(direction * player.walk_speed); // Adjust force magnitude as needed

        if !pressed {
            player.state = PlayerState::Stand;
        }
    }
}
