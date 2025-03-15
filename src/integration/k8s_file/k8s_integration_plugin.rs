use bevy::prelude::*;

use super::k8s_integration_systems::fire_k8s_file_floorplan_event;

pub struct K8sIntegrationPlugin;

impl Plugin for K8sIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fire_k8s_file_floorplan_event);
    }
}
