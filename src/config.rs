use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub url: String,
    pub explorers_counter: u32,
    pub diggers_count: u32,
    pub licensors_count: u32,
    pub bookkeeper_count: u32,
    pub licensor_rps_limit: u32,
    pub explorer_rps_limit: u32,
    pub digger_rps_limit: u32,
    pub bookkeeper_rps_limit: u32,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        let url = env::var("ADDRESS").unwrap_or("localhost".to_owned());
        let cfg_path = env::var("cfg-path").unwrap_or("".to_owned());
        cfg.merge(File::with_name(&cfg_path))?;
        cfg.set("url", format!("http://{}:8000", url))?;
        cfg.try_into()
    }
}
