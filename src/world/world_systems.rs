use super::world_component::{CurrentFloorPlan, Floor, PlatformMarker};
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

const ROOM_X_LEN: f32 = 4.0;
const ROOM_Y_LEN: f32 = 4.0;
const ROOM_Z_LEN: f32 = 4.0;
const PLACEHOLDER_Y_LEN: f32 = 0.1;

const FLOOR_THICKNESS: f32 = 0.5;
const N_ROWS: usize = 5;
const SPACING: f32 = 8.0;

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
        transition_to_next_state(&mut next_state);
    }
}

fn process_floorplan_event(
    current_floorplan: &mut CurrentFloorPlan,
    floorplan: &FloorPlan,
    time: &Res<Time>,
) -> bool {
    if current_floorplan.floorplan.as_ref() != Some(floorplan) {
        debug!("Floorplan changed");
        let you_are_here =
            determine_you_are_here(current_floorplan.you_are_here.as_ref(), floorplan);
        let you_were_here = current_floorplan.you_are_here.clone();

        *current_floorplan = CurrentFloorPlan {
            floorplan: Some(floorplan.clone()),
            refreshed: time.elapsed(),
            modified: time.elapsed(),
            you_are_here,
            you_were_here,
        };

        return true;
    }
    false
}

fn determine_you_are_here(
    current_you_are_here: Option<&Room>,
    floorplan: &FloorPlan,
) -> Option<Room> {
    if current_you_are_here.is_none() {
        if let Ok(start_room) = floorplan.get_start_room() {
            return Some(start_room.clone());
        }
    }
    current_you_are_here.cloned()
}

fn transition_to_next_state(next_state: &mut ResMut<NextState<GameState>>) {
    debug!("Transitioning");
    next_state.set(GameState::Transitioning);
}

pub fn spawn_world(
    mut commands: Commands,
    platform_query: Query<Entity, With<PlatformMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_floorplan: ResMut<CurrentFloorPlan>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(floorplan) = &current_floorplan.floorplan {
        // first, destroy all existing entities with the PlatformMarker component
        for entity in platform_query.iter() {
            commands.entity(entity).despawn();
        }

        // make a vec of the current room's connected room node_index
        let mut connected_room_node_index = Vec::new();
        if let Some(current_room) = &current_floorplan.you_are_here {
            if let Ok(entries) = &floorplan.get_doors_and_connected_rooms(&current_room.id) {
                for (_door, room) in entries {
                    if let Ok(node_index) = floorplan.get_room_idx_by_id(&room.id) {
                        connected_room_node_index.push(node_index);
                    }
                }
            }
        }

        spawn_floor(
            &mut commands,
            &mut meshes,
            &mut materials,
            floorplan.graph.node_indices().count(),
            5,
            8.0,
        );

        // Visualize Rooms
        for node_index in floorplan.graph.node_indices() {
            if let Some(room) = floorplan.graph.node_weight(node_index) {
                if connected_room_node_index.contains(&node_index) {
                    // is a connected room - we want to spawn a door
                    spawn_connected_room(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        node_index,
                        room,
                    );
                } else {
                    spawn_unconnected_room(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        node_index,
                        room,
                    );
                }
            }
        }

        next_state.set(GameState::InGame);
    }
}

fn spawn_connected_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_index: NodeIndex,
    room: &Room,
) {
    debug!("Spawning connected room");

    let shape = meshes.add(Cuboid::new(ROOM_X_LEN, ROOM_Y_LEN, ROOM_Z_LEN));
    let mat = materials.add(Color::from(calculate_room_color(&room.name)));
    let position = calculate_room_position(node_index, 1.8);
    let collider = Collider::cuboid(ROOM_X_LEN, ROOM_Y_LEN, ROOM_Z_LEN);

    let door = spawn_connected_room_door(commands, meshes, materials);

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
        .add_child(door);
}

fn spawn_connected_room_door(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    debug!("Spawning connected room door");

    let room_size = ROOM_Y_LEN;
    let door_size = Vec3::new(2.0, 3.0, 0.1); // Width, height, depth of the door
    let door_position = Vec3::new(0.0, 0.0, -(room_size / 2.0 + door_size.z / 2.0)); // Centered on the front face

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(door_size.x, door_size.y, door_size.z))),
            MeshMaterial3d(materials.add(Color::from(RED_600))),
            Transform::from_translation(door_position),
            RigidBody::Static,
            Collider::cuboid(door_size.x / 2.0, door_size.y / 2.0, door_size.z / 2.0),
            Door::default(), //TODO use correct component instance
            PlatformMarker::default(),
        ))
        .id()
}

fn spawn_unconnected_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_index: NodeIndex,
    room: &Room,
) {
    debug!("Spawning unconnected room");

    let shape = meshes.add(Cuboid::new(ROOM_X_LEN, PLACEHOLDER_Y_LEN, ROOM_Z_LEN));
    let mat = materials.add(Color::from(GRAY_600));
    let position = calculate_room_position(node_index, 0.0);
    let collider = Collider::cuboid(ROOM_X_LEN, PLACEHOLDER_Y_LEN, ROOM_Z_LEN);

    commands.spawn((
        Mesh3d(shape),
        MeshMaterial3d(mat),
        Transform::from_translation(position),
        room.clone(),
        RigidBody::Static,
        collider,
        PlatformMarker::default(),
    ));
}

#[allow(clippy::cast_precision_loss)]
fn spawn_floor(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    num_rooms: usize,
    columns: usize,
    room_spacing: f32,
) {
    let rows = (num_rooms as f32 / columns as f32).ceil();
    let floor_width = columns as f32 * room_spacing;
    let floor_depth = rows * room_spacing;
    let floor_thickness = FLOOR_THICKNESS;

    let floor_position = Vec3::new(
        (columns as f32 - 1.0) * room_spacing / 2.0,
        -floor_thickness / 2.0,
        (rows - 1.0) * room_spacing / 2.0,
    );

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(floor_width, floor_thickness, floor_depth))),
        MeshMaterial3d(materials.add(Color::from(GRAY_500))),
        Transform::from_translation(floor_position),
        RigidBody::Static,
        Collider::cuboid(floor_width, floor_thickness, floor_depth),
        Floor::default(),
        PlatformMarker::default(),
    ));
}

#[allow(clippy::cast_precision_loss)]
fn calculate_room_position(index: NodeIndex, yoffset: f32) -> Vec3 {
    let x = (index.index() % N_ROWS) as f32 * SPACING;
    let z = (index.index() / N_ROWS) as f32 * SPACING; // adjust 'spacing' as needed
    Vec3::new(x, 0.0 + yoffset, z)
}
