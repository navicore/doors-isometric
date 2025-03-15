use super::perf_component::SystemMonitor;
use bevy::prelude::*;
use iyes_perf_ui::prelude::PerfUiAllEntries;

pub fn toggle_builtins(
    mut commands: Commands,
    q_root: Query<Entity, With<SystemMonitor>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        if let Ok(e) = q_root.get_single() {
            commands.entity(e).despawn_recursive();
        } else {
            commands.spawn((SystemMonitor, PerfUiAllEntries::default()));
        }
    }
}
