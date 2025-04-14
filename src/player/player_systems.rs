use super::player_component::{
    Action, Grounded, GroundedState, Player, PlayerBundle, PlayerConfig, PlayerDirection,
    PlayerStartPosition, PlayerState,
};
use crate::{
    floorplan::{Door, Room},
    state::{state_component::GameOverReason, GameState},
    world::world_component::{CurrentFloorPlan, Floor, PlatformMarker, Wall, WallState},
};
use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*};
use leafwing_input_manager::prelude::ActionState;

pub fn spawn_player(
    player_config: Res<PlayerConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
    start_position: Res<PlayerStartPosition>,
) {
    let player_shape = meshes.add(Sphere::new(player_config.x / 2.0));
    let player_material = materials.add(Color::from(BLUE_600));
    let spawn_position = start_position.position.unwrap_or(Vec3::ZERO)
        + Vec3 {
            x: 0.0,
            y: 8.0,
            z: -0.5,
        };

    commands.spawn((
        Mesh3d(player_shape),
        MeshMaterial3d(player_material),
        PlayerBundle::new(&PlayerConfig::default(), spawn_position),
    ));
    next_state.set(GameState::InGame);
}

pub fn player_movement(
    player_config: Res<PlayerConfig>,
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
            direction.y += player_config.jump_force;
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
pub fn detect_grounded(
    mut next_state: ResMut<NextState<GameState>>,
    floor_query: Query<&Transform, With<Floor>>,
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

        // Check if the player is too far below the floor
        if let Ok(floor_transform) = floor_query.get_single() {
            let player_y = player_transform.translation.y;
            let floor_y = floor_transform.translation.y;

            if player_y < floor_y - 200.0 {
                next_state.set(GameState::GameOver {
                    reason: GameOverReason::PlayerFell,
                });
                debug!("Player fell off the platform. Transitioning to GameOver.");
            }
        }
    }

    if !t_grounded {
        debug!("Player is not grounded");
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn detect_enter_door(
    mut command: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
    mut collision_events: EventReader<Collision>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    door_query: Query<(Entity, &Transform, &Parent, &Door)>,
    room_query: Query<&Room>,
    mut start_position: ResMut<PlayerStartPosition>,
) {
    if let Ok((player, transform)) = player_query.get_single_mut() {
        for collision in collision_events.read() {
            if let Some(room_entity) = find_door_collision(collision, &door_query) {
                if let Ok(room) = room_query.get(room_entity) {
                    *current_floorplan = CurrentFloorPlan {
                        floorplan: current_floorplan.floorplan.clone(),
                        you_are_here: Some(room.clone()),
                        previous_room: current_floorplan.you_are_here.clone(),
                        ..Default::default()
                    };

                    debug!("Entering room: {:?}", room);
                    start_position.position = Some(transform.translation);

                    command.entity(player).despawn();
                    next_state.set(GameState::TransitioningOutSetup);
                }
            }
        }
    }
}

pub fn detect_wall_collision(
    mut wall_query: Query<(&mut WallState, &mut Visibility), With<Wall>>,
    mut collision_events: EventReader<Collision>,
) {
    for collision in collision_events.read() {
        let contacts = &collision.0;
        if contacts.is_sensor {
            continue;
        }
        let involved_entities = [contacts.entity1, contacts.entity2];

        for entity in &involved_entities {
            if let Ok((mut state, mut visibility)) = wall_query.get_mut(*entity) {
                *state = WallState::Visible(Timer::from_seconds(1.0, TimerMode::Once));
                *visibility = Visibility::Visible;
            }
        }
    }
}
