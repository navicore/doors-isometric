use integration::integration_plugin::IntegrationPlugin;
mod integration;
use bevy::prelude::*;
use camera::IsometricCameraPlugin;
use clap::Parser;
#[cfg(feature = "perfmon")]
use perf::PerfPlugin;
mod camera;
mod cli;
mod floorplan;
mod perf;

fn main() {
    cli::Cli::parse();

    App::new()
        .add_plugins((
            DefaultPlugins,
            IsometricCameraPlugin,
            IntegrationPlugin,
            #[cfg(feature = "perfmon")]
            PerfPlugin,
        ))
        .run();
}
