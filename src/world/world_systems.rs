use super::world_component::CurrentFloorPlan;
use crate::{floorplan::FloorPlanEvent, state::GameState};
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
                let (shape, mat, position, collider) =
                    if connected_room_node_index.contains(&node_index) {
                        (
                            meshes.add(Cuboid::new(4.0, 2.0, 4.0)),
                            materials.add(Color::from(calculate_room_color(&room.name))),
                            calculate_room_position(node_index, 0.8),
                            Collider::cuboid(4.0, 2.0, 4.0),
                        )
                    } else {
                        (
                            meshes.add(Cuboid::new(4.0, 0.1, 4.0)),
                            materials.add(Color::from(GRAY_600)),
                            calculate_room_position(node_index, 0.0),
                            Collider::cuboid(4.0, 0.1, 4.0),
                        )
                    };

                commands.spawn((
                    Mesh3d(shape),
                    MeshMaterial3d(mat),
                    Transform::from_translation(position),
                    room.clone(),
                    collider, // Add collider to the room
                ));
            }
        }

        // Visualize Doors (Edges)
        // for edge in floorplan.graph.edge_references() {
        //     let source_pos = calculate_room_position(edge.source());
        //     let target_pos = calculate_room_position(edge.target());
        //
        //     let door_pos = (source_pos + target_pos) / 2.0;
        //     let direction = target_pos - source_pos;
        //
        //     let shape = meshes.add(Cuboid::new(1.0, 1.5, 0.2));
        //     let mat = materials.add(Color::from(RED_500));
        //     commands.spawn((
        //         Mesh3d(shape),
        //         MeshMaterial3d(mat),
        //         Transform {
        //             translation: door_pos,
        //             rotation: Quat::from_rotation_y(direction.z.atan2(direction.x)),
        //             ..default()
        //         },
        //         edge.weight().clone(),
        //     ));
        // }

        next_state.set(GameState::InGame);
    }
}

#[allow(clippy::cast_precision_loss)]
fn calculate_room_position(index: NodeIndex, yoffset: f32) -> Vec3 {
    // For simplicity, arrange rooms in a grid pattern
    let spacing = 5.0;
    let x = (index.index() % 5) as f32 * spacing;
    let z = (index.index() / 5) as f32 * spacing; // adjust 'spacing' as needed
    Vec3::new(x, 0.0 + yoffset, z)
}
