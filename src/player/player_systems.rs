use crate::floorplan::Room;

use super::player_component::{
    Action, Grounded, GroundedState, Player, PlayerBundle, PlayerDirection, PlayerState,
    PLAYER_JUMP_FORCE, PLAYER_SHAPE_X, PLAYER_SHAPE_Y, PLAYER_SHAPE_Z,
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

        pressed = if action_state.pressed(&Action::Jump) {
            direction.x += 0.0;
            direction.y += PLAYER_JUMP_FORCE;
            direction.z += 0.0;
            player.direction = PlayerDirection::Up; // ?
            player.state = PlayerState::Jump;
            true
        } else {
            pressed
        };

        if pressed && grounded.0 {
            force.apply_force(direction * player.walk_speed); // Adjust force magnitude as needed
        } else {
            force.set_force(Vec3::ZERO);
        }

        if !pressed {
            player.state = PlayerState::Stand;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_grounded(
    mut collision_events: EventReader<Collision>,
    mut grounded_state: ResMut<GroundedState>,
    mut query: Query<(Entity, &mut Grounded, &Transform), With<Player>>,
    platform_query: Query<(Entity, &Transform), (With<Room>, Without<Player>)>, // Query for platforms
) {
    let player_entities: Vec<Entity> = query.iter().map(|(entity, _, _)| entity).collect();

    let mut t_grounded = false;

    if let Ok((_, grounded, player_transform)) = &mut query.get_single_mut() {
        for collision in collision_events.read() {
            let contacts = &collision.0;

            if contacts.is_sensor {
                continue;
            }

            let involved_entities = [contacts.entity1, contacts.entity2];
            if !involved_entities
                .iter()
                .any(|e| player_entities.contains(e))
            {
                continue;
            }

            for entity in &involved_entities {
                if let Ok((_, room_transform)) = platform_query.get(*entity) {
                    if player_transform.translation.y > room_transform.translation.y {
                        t_grounded = true;
                    }
                }
            }
        }
        grounded.0 = t_grounded;
    }
    grounded_state.0 = t_grounded;
}
