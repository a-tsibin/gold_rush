extern crate log;

use env_logger::Target;
use log::LevelFilter;

use crate::config::AppConfig;
use crate::game_logic::GameLogic;
use log::info;

mod client;
mod config;
mod data;
mod game_logic;
mod workers;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Warn)
        .target(Target::Stdout)
        .init();
    run().await
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("Started");
    let config = AppConfig::new()?;
    info!("{:?}", config);
    let logic = GameLogic::new(config);
    logic.start().await
}
