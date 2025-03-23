use super::perf_component::PlayerIsGrounded;
use super::perf_component::RoomName;
use super::perf_component::SystemMonitor;
use super::perf_component::TimeInRoom;
use bevy::prelude::*;
use iyes_perf_ui::prelude::PerfUiAllEntries;
use iyes_perf_ui::prelude::PerfUiPosition;
use iyes_perf_ui::prelude::PerfUiRoot;

use super::perf_component::GameWorldMonitor;
use super::perf_component::TimeSinceLastFloorplanModified;
use super::perf_component::TimeSinceLastFloorplanRefresh;
use super::{WorldEdgeCount, WorldNodeCount};

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

pub fn toggle_customs(
    mut commands: Commands,
    q_root: Query<Entity, With<GameWorldMonitor>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F10) {
        if let Ok(e) = q_root.get_single() {
            commands.entity(e).despawn_recursive();
        } else {
            commands.spawn((
                PerfUiRoot {
                    position: PerfUiPosition::TopLeft,
                    ..Default::default()
                },
                GameWorldMonitor,
                WorldNodeCount::default(),
                WorldEdgeCount::default(),
                TimeSinceLastFloorplanRefresh::default(),
                TimeSinceLastFloorplanModified::default(),
                TimeInRoom::default(),
                RoomName::default(),
                PlayerIsGrounded::default(),
            ));
        }
    }
}
