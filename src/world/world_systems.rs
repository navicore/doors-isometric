use crate::floorplan::FloorPlanEvent;
use bevy::{
    color::palettes::tailwind::{GRAY_600, RED_500},
    prelude::*,
};
use petgraph::prelude::*;

pub fn handle_floor_plan_event(
    mut commands: Commands,
    mut events: EventReader<FloorPlanEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        let floorplan = &event.floorplan;

        // Visualize Rooms
        for node_index in floorplan.graph.node_indices() {
            if let Some(room) = floorplan.graph.node_weight(node_index) {
                let position = calculate_room_position(node_index);

                let shape = meshes.add(Cuboid::new(4.0, 1.5, 4.0));
                let mat = materials.add(Color::from(GRAY_600));
                commands.spawn((
                    Mesh3d(shape),
                    MeshMaterial3d(mat),
                    Transform::from_translation(position),
                    room.clone(),
                ));
            }
        }

        // Visualize Doors (Edges)
        for edge in floorplan.graph.edge_references() {
            let source_pos = calculate_room_position(edge.source());
            let target_pos = calculate_room_position(edge.target());

            let door_pos = (source_pos + target_pos) / 2.0;
            let direction = target_pos - source_pos;

            let shape = meshes.add(Cuboid::new(1.0, 1.5, 0.2));
            let mat = materials.add(Color::from(RED_500));
            commands.spawn((
                Mesh3d(shape),
                MeshMaterial3d(mat),
                Transform {
                    translation: door_pos,
                    rotation: Quat::from_rotation_y(direction.z.atan2(direction.x)),
                    ..default()
                },
                edge.weight().clone(),
            ));
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn calculate_room_position(index: NodeIndex) -> Vec3 {
    // For simplicity, arrange rooms in a grid pattern
    let spacing = 6.0;
    let x = (index.index() % 5) as f32 * spacing;
    let z = (index.index() / 5) as f32 * spacing; // adjust 'spacing' as needed
    Vec3::new(x, 0.0, z)
}
