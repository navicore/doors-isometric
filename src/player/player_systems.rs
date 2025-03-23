use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::BLUE_600, prelude::*};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_shape = meshes.add(Cuboid::new(0.5, 1.0, 0.5)); // Player size
    let player_material = materials.add(Color::from(BLUE_600));
    commands.spawn((
        Mesh3d(player_shape),
        MeshMaterial3d(player_material),
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)), // Start above the floor
        RigidBody::Dynamic,              // Dynamic body for physics interactions
        Collider::cuboid(0.5, 1.0, 0.5), // Collider matching the player size
    ));
}

// pub fn player_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<&mut RigidBodyVelocity, With<RigidBody>>,
// ) {
//     for mut velocity in query.iter_mut() {
//         let mut direction = Vec3::ZERO;
//         if keyboard_input.pressed(KeyCode::W) {
//             direction.z -= 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::S) {
//             direction.z += 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::A) {
//             direction.x -= 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::D) {
//             direction.x += 1.0;
//         }
//
//         // Normalize direction and apply velocity
//         if direction != Vec3::ZERO {
//             direction = direction.normalize();
//         }
//         velocity.linvel = direction * 5.0; // Adjust speed as needed
//     }
// }
