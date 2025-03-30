use super::player_component::{
    Action, Grounded, GroundedState, Player, PlayerBundle, PlayerDirection, PlayerState,
    PLAYER_JUMP_FORCE, PLAYER_SHAPE_X,
};
use crate::{
    floorplan::{Door, Room},
    state::GameState,
    world::world_component::{CurrentFloorPlan, PlatformMarker},
};
use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*};
use leafwing_input_manager::prelude::ActionState;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_shape = meshes.add(Sphere::new(PLAYER_SHAPE_X / 2.0));
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
    mut player_query: Query<(Entity, &mut Grounded, &Transform), With<Player>>,
    platform_query: Query<(Entity, &Transform), With<PlatformMarker>>,
) {
    let mut t_grounded = false;

    if let Ok((player_entity, grounded, player_transform)) = &mut player_query.get_single_mut() {
        for collision in collision_events.read() {
            let contacts = &collision.0;

            if contacts.is_sensor {
                continue;
            }

            let involved_entities = [contacts.entity1, contacts.entity2];
            if !involved_entities.iter().any(|e| e == player_entity) {
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

#[allow(clippy::type_complexity)]
fn find_door_collision(
    collision: &Collision,
    door_query: &Query<(Entity, &Transform, &Parent, &Door)>,
) -> Option<Entity> {
    let contacts = &collision.0;
    let involved_entities = [contacts.entity1, contacts.entity2];

    if contacts.is_sensor {
        return None;
    }

    for entity in &involved_entities {
        if let Ok((_entity, _transform, parent, _door)) = door_query.get(*entity) {
            return Some(parent.get());
        }
    }

    None
}

#[allow(clippy::type_complexity)]
pub fn detect_enter_door(
    mut next_state: ResMut<NextState<GameState>>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
    mut collision_events: EventReader<Collision>,
    door_query: Query<(Entity, &Transform, &Parent, &Door)>,
    room_query: Query<&Room>,
) {
    for collision in collision_events.read() {
        if let Some(room_entity) = find_door_collision(collision, &door_query) {
            handle_door_entry(
                room_entity,
                &mut current_floorplan,
                &room_query,
                &mut next_state,
            );
        }
    }
}

fn handle_door_entry(
    room_entity: Entity,
    current_floorplan: &mut CurrentFloorPlan,
    room_query: &Query<&Room>,
    next_state: &mut ResMut<NextState<GameState>>,
) {
    if let Ok(room) = room_query.get(room_entity) {
        *current_floorplan = CurrentFloorPlan {
            floorplan: current_floorplan.floorplan.clone(),
            you_are_here: Some(room.clone()),
            previous_room: current_floorplan.you_are_here.clone(),
            ..Default::default()
        };

        next_state.set(GameState::TransitioningOut);
    }
}
