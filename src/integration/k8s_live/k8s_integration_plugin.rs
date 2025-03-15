use bevy::prelude::*;

use super::k8s_integration_systems::init_k8s_live_floorplan_publisher;

pub struct K8sIntegrationPlugin;

impl Plugin for K8sIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_k8s_live_floorplan_publisher);
    }
}
