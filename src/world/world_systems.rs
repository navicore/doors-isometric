use super::world_component::CurrentFloorPlan;
use crate::{
    floorplan::{FloorPlanEvent, Room},
    state::GameState,
};
use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::{
        BLUE_600, GRAY_600, GREEN_600, ORANGE_600, PURPLE_600, RED_600, YELLOW_600,
    },
    prelude::*,
};
use petgraph::prelude::*;

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

#[allow(clippy::cognitive_complexity)]
pub fn handle_floor_plan_event(
    mut events: EventReader<FloorPlanEvent>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut should_transition = false;
    for event in events.read() {
        let floorplan = &event.floorplan;
        debug!("Handling floorplan event");

        if current_floorplan.floorplan.as_ref() != Some(floorplan) {
            debug!("Floorplan changed");
            let mut you_are_here = current_floorplan.you_are_here.clone();
            if you_are_here.is_none() {
                if let Ok(start_room) = floorplan.get_start_room() {
                    you_are_here = Some(start_room.clone());
                }
            }
            let you_were_here = current_floorplan.you_are_here.clone();

            *current_floorplan = CurrentFloorPlan {
                floorplan: Some(floorplan.clone()),
                refreshed: time.elapsed(),
                modified: time.elapsed(),
                you_are_here,
                you_were_here,
            };
            should_transition = true;
        }
    }
    if should_transition {
        debug!("Transitioning");
        next_state.set(GameState::Transitioning);
    }
}

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_floorplan: ResMut<CurrentFloorPlan>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    debug!("Spawning world");
    if let Some(floorplan) = &current_floorplan.floorplan {
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

pub fn spawn_connected_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_index: NodeIndex,
    room: &Room,
) {
    debug!("Spawning connected room");

    let shape = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
    let mat = materials.add(Color::from(calculate_room_color(&room.name)));
    let position = calculate_room_position(node_index, 1.8);
    let collider = Collider::cuboid(4.0, 4.0, 4.0);

    let door = spawn_connected_room_door(commands, meshes, materials);

    commands
        .spawn((
            Mesh3d(shape),
            MeshMaterial3d(mat),
            Transform::from_translation(position),
            room.clone(),
            RigidBody::Static,
            collider,
        ))
        .add_child(door);
}

pub fn spawn_connected_room_door(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    debug!("Spawning connected room door");

    let room_size = 4.0;
    let door_size = Vec3::new(2.0, 3.0, 0.1); // Width, height, depth of the door
    let door_position = Vec3::new(0.0, 0.0, -(room_size / 2.0 + door_size.z / 2.0)); // Centered on the front face

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(door_size.x, door_size.y, door_size.z))),
            MeshMaterial3d(materials.add(Color::from(RED_600))),
            Transform::from_translation(door_position),
            Collider::cuboid(door_size.x / 2.0, door_size.y / 2.0, door_size.z / 2.0),
        ))
        .id()
}

pub fn spawn_unconnected_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_index: NodeIndex,
    room: &Room,
) {
    debug!("Spawning unconnected room");

    let shape = meshes.add(Cuboid::new(4.0, 0.1, 4.0));
    let mat = materials.add(Color::from(GRAY_600));
    let position = calculate_room_position(node_index, 0.0);
    let collider = Collider::cuboid(4.0, 0.1, 4.0);

    commands.spawn((
        Mesh3d(shape),
        MeshMaterial3d(mat),
        Transform::from_translation(position),
        room.clone(),
        RigidBody::Static,
        collider, // Add collider to the room
    ));
}

#[allow(clippy::cast_precision_loss)]
fn calculate_room_position(index: NodeIndex, yoffset: f32) -> Vec3 {
    // For simplicity, arrange rooms in a grid pattern
    //let spacing = 5.0;
    let spacing = 4.2;
    let x = (index.index() % 5) as f32 * spacing;
    let z = (index.index() / 5) as f32 * spacing; // adjust 'spacing' as needed
    Vec3::new(x, 0.0 + yoffset, z)
}
