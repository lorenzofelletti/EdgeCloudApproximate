use std::{path::Path, time::Duration, fs};

use toml::Value;

use self::errors::ConfigError;

pub mod errors;

#[derive(Debug)]
pub struct Kafka {
    pub brokers: Vec<String>,
}

#[derive(Debug)]
pub struct Data<'a> {
    pub source: &'a Path,
    pub msg_sleep_in_ms: Duration,
    chunk_size: u32,
    chunk_sleep_in_ms: Duration,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub kafka: Kafka,
    pub data: Data<'a>,
}

fn load_config<'a>(from: &Path) -> Result<Config<'a>, ConfigError> {
    let contents = fs::read_to_string(from).expect("Config file `producer_config.toml` should exists!");

    let config: Value = toml::from_str(&contents).map_err(|e| ConfigError::new(e.message()))?;

    Err(ConfigError::new("Error parsing the configuration"))
}
