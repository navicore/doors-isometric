use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use super::player_component::{Action, Player};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_shape = meshes.add(Cuboid::new(0.5, 1.0, 0.5)); // Player size
    let player_material = materials.add(Color::from(BLUE_600));

    let input_map = InputMap::new([
        (Action::MoveForward, KeyCode::ArrowUp),
        (Action::MoveBackward, KeyCode::ArrowDown),
        (Action::MoveLeft, KeyCode::ArrowLeft),
        (Action::MoveRight, KeyCode::ArrowRight),
        (Action::Jump, KeyCode::Space),
        (Action::Enter, KeyCode::Enter),
    ]);

    commands.spawn((
        Mesh3d(player_shape),
        MeshMaterial3d(player_material),
        Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)), // Start above the floor
        RigidBody::Dynamic,              // Dynamic body for physics interactions
        Collider::cuboid(0.5, 1.0, 0.5), // Collider matching the player size
        ExternalForce::default(),
        Player::default(),
        InputManagerBundle::with_map(input_map),
    ));
}

pub fn player_movement(mut query: Query<(&mut ExternalForce, &ActionState<Action>), With<Player>>) {
    if let Ok((mut force, action_state)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if action_state.pressed(&Action::MoveForward) {
            direction.z += 1.0;
        }
        if action_state.pressed(&Action::MoveBackward) {
            direction.z -= 1.0;
        }
        if action_state.pressed(&Action::MoveLeft) {
            direction.x += 1.0;
        }
        if action_state.pressed(&Action::MoveRight) {
            direction.x -= 1.0;
        }

        // Normalize direction and apply force
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        force.apply_force(direction * 1.0); // Adjust force magnitude as needed
    }
}
