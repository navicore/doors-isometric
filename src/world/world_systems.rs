use super::world_component::{
    CurrentFloorPlan, Floor, PlatformMarker, PlatformTransition, WorldConfig,
};
use crate::{
    floorplan::{Door, FloorPlan, FloorPlanEvent, Room},
    state::GameState,
};
use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::{
        BLUE_600, GRAY_500, GRAY_600, GREEN_600, ORANGE_600, PURPLE_600, RED_600, YELLOW_600,
    },
    prelude::*,
};
use petgraph::prelude::*;
use std::collections::HashMap;

fn calculate_room_color(name: &str) -> Srgba {
    match name {
        n if n.contains("Deployment") => PURPLE_600,
        n if n.contains("ReplicaSet") => ORANGE_600,
        n if n.contains("Pod") => RED_600,
        n if n.contains("Service") => BLUE_600,
        n if n.contains("CofnigMap") => YELLOW_600,
        n if n.contains("Hallway") => GREEN_600,
        _ => GRAY_600,
    }
}

pub fn handle_floor_plan_event(
    mut events: EventReader<FloorPlanEvent>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut should_transition = false;

    for event in events.read() {
        if process_floorplan_event(&mut current_floorplan, &event.floorplan, &time) {
            should_transition = true;
        }
    }
    if should_transition {
        debug!("Transitioning to new floorplan");
        next_state.set(GameState::TransitioningOutSetup);
    }
}

fn process_floorplan_event(
    current_floorplan: &mut CurrentFloorPlan,
    floorplan: &FloorPlan,
    time: &Res<Time>,
) -> bool {
    if current_floorplan.floorplan.is_none() {
        current_floorplan.floorplan = Some(floorplan.clone());
        current_floorplan.you_are_here = determine_you_are_here(floorplan);
        current_floorplan.previous_room = None;
        return true;
    }
    // if current floor plan has changed then we need to update the current floor plan
    if let Some(plan) = &current_floorplan.floorplan {
        if plan != floorplan {
            current_floorplan.floorplan = Some(floorplan.clone());
            current_floorplan.modified = time.elapsed();
            return true;
        }
    }

    false
}

fn determine_you_are_here(floorplan: &FloorPlan) -> Option<Room> {
    floorplan
        .get_start_room()
        .map_or(None, |start_room| Some(start_room.clone()))
}

pub fn transition_in_setup(
    world_config: Res<WorldConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_floorplan: ResMut<CurrentFloorPlan>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let initial_y_offset = 1.5;

    if let Some(floorplan) = &current_floorplan.floorplan {
        let previous_room = current_floorplan.previous_room.clone();

        let mut connected_rooms_and_doors = HashMap::new();
        if let Some(current_room) = &current_floorplan.you_are_here {
            if let Ok(entries) = floorplan.get_doors_and_connected_rooms(&current_room.id) {
                for (door, room) in entries {
                    if let Ok(node_index) = floorplan.get_room_idx_by_id(&room.id) {
                        connected_rooms_and_doors.insert(node_index, door);
                    }
                }
            }
        }
        let num_rooms = floorplan.graph.node_indices().count();

        let floor_entity: Entity = spawn_floor(
            &world_config,
            &mut commands,
            &mut meshes,
            &mut materials,
            floorplan.graph.node_indices().count(),
            world_config.n_columns,
            world_config.spacing,
            initial_y_offset,
        );

        // Visualize Rooms
        for node_index in floorplan.graph.node_indices() {
            if let Some(room) = floorplan.graph.node_weight(node_index) {
                if connected_rooms_and_doors.contains_key(&node_index) {
                    if let Some(door) = connected_rooms_and_doors.remove(&node_index) {
                        let is_exit = previous_room
                            .as_ref()
                            .is_some_and(|previous_room| previous_room.id == room.id);
                        // is a connected room - we want to spawn a door
                        let room_entity: Entity = spawn_connected_room(
                            &world_config,
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            node_index,
                            num_rooms,
                            room,
                            door.clone(),
                            is_exit,
                            initial_y_offset + world_config.room_y / 2.0,
                        );
                        commands.entity(floor_entity).add_child(room_entity);
                    }
                } else {
                    let room_entity: Entity = spawn_unconnected_room(
                        &world_config,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        node_index,
                        num_rooms,
                        room,
                        initial_y_offset,
                    );
                    commands.entity(floor_entity).add_child(room_entity);
                }
            }
        }

        debug!("Spawned world with {} rooms", floorplan.graph.node_count());
    }
    next_state.set(GameState::TransitioningIn);
}

#[allow(clippy::too_many_arguments)]
fn spawn_connected_room(
    world_config: &WorldConfig,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_index: NodeIndex,
    num_rooms: usize,
    room: &Room,
    door: Door,
    is_exit: bool, // Whether this room is the previous room
    initial_y_offset: f32,
) -> Entity {
    let room_height = if is_exit {
        world_config.exit_room_y
    } else {
        world_config.room_y
    };

    let shape = meshes.add(Cuboid::new(
        world_config.room_x,
        room_height,
        world_config.room_z,
    ));
    let mat = materials.add(Color::from(calculate_room_color(&room.name)));
    let position = calculate_room_position(world_config, node_index, initial_y_offset, num_rooms);
    let collider = Collider::cuboid(world_config.room_x, room_height, world_config.room_z);

    let door = spawn_connected_room_door(world_config, commands, meshes, materials, door);

    commands
        .spawn((
            Mesh3d(shape),
            MeshMaterial3d(mat),
            Transform::from_translation(position),
            room.clone(),
            RigidBody::Static,
            collider,
            PlatformMarker::default(),
        ))
        .add_child(door)
        .id()
}

fn spawn_connected_room_door(
    world_config: &WorldConfig,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    door: Door,
) -> Entity {
    debug!("Spawning connected room door");

    let room_size = world_config.room_y;
    let door_size = Vec3::new(2.0, 3.8, 0.1); // Width, height, depth of the door
    let door_position = Vec3::new(0.0, 0.0, -(room_size / 2.0 + door_size.z / 2.0)); // Centered on the front face

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(door_size.x, door_size.y, door_size.z))),
            MeshMaterial3d(materials.add(Color::from(RED_600))),
            Transform::from_translation(door_position),
            RigidBody::Static,
            Collider::cuboid(door_size.x / 2.0, door_size.y, door_size.z / 2.0),
            door,
            PlatformMarker::default(),
        ))
        .id()
}

#[allow(clippy::too_many_arguments)]
fn spawn_unconnected_room(
    world_config: &WorldConfig,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_index: NodeIndex,
    num_rooms: usize,
    room: &Room,
    initial_y_offset: f32,
) -> Entity {
    debug!("Spawning unconnected room");

    let shape = meshes.add(Cuboid::new(
        world_config.room_x,
        world_config.placeholder_y,
        world_config.room_z,
    ));
    let mat = materials.add(Color::from(GRAY_600));
    let position =
        calculate_room_position(world_config, node_index, 0.0 + initial_y_offset, num_rooms);
    let collider = Collider::cuboid(
        world_config.room_x,
        world_config.placeholder_y,
        world_config.room_z,
    );

    commands
        .spawn((
            Mesh3d(shape),
            MeshMaterial3d(mat),
            Transform::from_translation(position),
            room.clone(),
            RigidBody::Static,
            collider,
            PlatformMarker::default(),
        ))
        .id()
}

#[allow(clippy::too_many_arguments)]
fn spawn_floor(
    world_config: &WorldConfig,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    num_rooms: usize,
    columns: usize,
    room_spacing: f32,
    y_offset: f32,
) -> Entity {
    let rows = (num_rooms as f32 / columns as f32).ceil();
    let floor_width = columns as f32 * room_spacing;
    let floor_depth = rows * room_spacing;
    let floor_thickness = world_config.floor_thickness;
    let floor_offset = 100.0;

    let floor_position = Vec3::new(
        (columns as f32 - 1.0) * room_spacing / 2.0,
        -floor_thickness / 2.0 + y_offset - floor_offset,
        (rows - 1.0) * room_spacing / 2.0,
    );

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(floor_width, floor_thickness, floor_depth))),
            MeshMaterial3d(materials.add(Color::from(GRAY_500))),
            Transform::from_translation(floor_position),
            RigidBody::Static,
            Collider::cuboid(floor_width, floor_thickness, floor_depth),
            Floor::default(),
            PlatformMarker::default(),
            PlatformTransition {
                target_y: 0.0,
                speed: 0.5, // Adjust speed as needed
            },
        ))
        .id()
}

#[allow(clippy::cast_precision_loss)]
fn calculate_room_position(
    world_config: &WorldConfig,
    index: NodeIndex,
    yoffset: f32,
    num_rooms: usize,
) -> Vec3 {
    let n_rows = (num_rooms as f32 / world_config.n_columns as f32).ceil() as usize;
    let column = index.index() % world_config.n_columns;
    let row = index.index() / world_config.n_columns;

    let x = column as f32 * world_config.spacing
        - (world_config.n_columns as f32 - 1.0) * world_config.spacing / 2.0;

    let z =
        (row as f32 + 0.5) * world_config.spacing - (n_rows as f32 * world_config.spacing / 2.0);

    // Align with the floor's y_offset
    Vec3::new(x, yoffset, z)
}

pub fn platform_transitioning_in(
    mut query: Query<(&mut Transform, &PlatformTransition)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut transitions_remaining = false;

    for (mut transform, transition) in &mut query {
        // Move the entity upward
        transform.translation.y += transition.speed;

        // Check if the platform is off-screen
        if transform.translation.y < transition.target_y {
            // At least one platform object is still transitioning
            transitions_remaining = true;
        }
    }

    if !transitions_remaining {
        next_state.set(GameState::TransitioningComplete);
    }
}

/// system to mark the current platform entities for transition
pub fn transition_out_setup(
    platform_query: Query<(Entity, &Transform), With<Floor>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Animate existing platforms to rise out of view
    for (entity, transform) in platform_query.iter() {
        commands.entity(entity).insert(PlatformTransition {
            target_y: transform.translation.y + 100.0, // Move up by 1000 units
            speed: 0.5,                                // Adjust speed as needed
        });
    }

    next_state.set(GameState::TransitioningOut);
}

/// system to animate the transitioning out of current platform entities
pub fn platform_transitioning_out(
    mut query: Query<(Entity, &mut Transform, &PlatformTransition)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut transitions_remaining = false;

    for (entity, mut transform, transition) in &mut query {
        // Move the entity upward
        transform.translation.y += transition.speed;

        // Check if the platform is off-screen
        if transform.translation.y > transition.target_y {
            // Transition is complete for this entity
            commands.entity(entity).despawn();
        } else {
            // At least one platform object is still transitioning
            transitions_remaining = true;
        }
    }

    if !transitions_remaining {
        next_state.set(GameState::TransitioningInSetup);
    }
}
