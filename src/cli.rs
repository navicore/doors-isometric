use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long, short, default_value = "demo1")]
    pub player: Option<PlayerType>,
    #[arg(long, short, default_value = "rooms5")]
    pub room_generator: Option<RoomGeneratorType>,
    #[arg(long, short, default_value = "60")]
    pub generator_poll_secs: Option<u8>,
}

#[derive(clap::ValueEnum, Clone, Default)]
pub enum PlayerType {
    #[default]
    Demo1,
    Player1,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoomGeneratorType {
    Rooms2,
    #[default]
    Rooms5,
    Rooms25,
    K8sFile,
    K8sLive,
}
