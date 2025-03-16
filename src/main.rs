use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_tokio_tasks::TokioTasksPlugin;
use floorplan::FloorPlanEvent;
use integration::integration_plugin::IntegrationPlugin;
mod integration;
use bevy::prelude::*;
use camera::IsometricCameraPlugin;
use clap::Parser;
#[cfg(feature = "perfmon")]
use perf::PerfPlugin;
use state::StatePlugin;
use world::WorldPlugin;
mod camera;
mod cli;
mod floorplan;
mod perf;
mod state;
mod world;

fn main() {
    cli::Cli::parse();

    App::new()
        .add_event::<FloorPlanEvent>()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins,
            TokioTasksPlugin::default(),
            IsometricCameraPlugin,
            IntegrationPlugin,
            WorldPlugin,
            #[cfg(feature = "perfmon")]
            PerfPlugin,
            StatePlugin,
            //InputManagerPlugin::<Action>::default(),
        ))
        .run();
}
