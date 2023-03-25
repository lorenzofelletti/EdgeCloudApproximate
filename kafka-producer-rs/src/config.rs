use std::{fs, num::NonZeroU64, path::Path, time::Duration};

use toml::Value;

use self::errors::{ConfigurationError, ErrorType};

pub mod errors;

#[derive(Debug, Clone)]
pub struct Kafka {
    pub brokers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Data<'a> {
    pub source: &'a Path,
    pub msg_sleep_in_ms: Duration,
    chunk_size: NonZeroU64,
    chunk_sleep_in_ms: Duration,
}

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub kafka: Kafka,
    pub data: Data<'a>,
}

fn check_brokers(brokers: &Vec<String>) -> Result<(), ConfigurationError> {
    Ok(())
}

pub fn load_config<'a>(from: &Path) -> Result<Config<'a>, ConfigurationError> {
    let contents =
        fs::read_to_string(from).expect("Config file `producer_config.toml` should exists!");

    let config: Value = toml::from_str(&contents)
        .map_err(|e| ConfigurationError::new(e.message(), ErrorType::Error))?;

    // Parses 'kakfa' table
    let kafka = config["kafka"]
        .as_table()
        .ok_or(ConfigurationError::new("kafka", ErrorType::TableNotFound))?;

    let brokers = kafka["brokers"].as_array().ok_or(ConfigurationError::new(
        "brokers",
        ErrorType::KeyNotFoundForTable("kafka"),
    ))?;
    let brokers: Vec<String> = brokers
        .into_iter()
        .map(|x| x.as_str().unwrap_or_default().to_owned())
        .collect();
    if brokers.contains(&String::from("")) {
        return Err(ConfigurationError::new(
            "",
            ErrorType::InvalidValueForKey("brokers"),
        ));
    }

    let kafka = Kafka { brokers };

    // Parses 'data' table
    let data = config["data"]
        .as_table()
        .ok_or(ConfigurationError::new("data", ErrorType::TableNotFound))?;

    let source = data["source"].as_str().ok_or(ConfigurationError::new(
        "source",
        ErrorType::KeyNotFoundForTable("data"),
    ))?;
    let source = Path::new(source);

    let msg_sleep_in_ms: u64 = data["msg_sleep_in_ms"]
        .as_integer()
        .ok_or(ConfigurationError::new(
            "msg_sleep_in_ms",
            ErrorType::KeyNotFoundForTable("data"),
        ))?
        .try_into()
        .map_err(|e: <u64 as TryFrom<i64>>::Error| ConfigurationError::Error(e.to_string()))?;
    let msg_sleep_in_ms = Duration::from_millis(msg_sleep_in_ms);

    let chunk_size = data["chunk_size"]
        .as_integer()
        .ok_or(ConfigurationError::new(
            "chunk_size",
            ErrorType::KeyNotFoundForTable("data"),
        ))?;
    let chunk_size: u64 = chunk_size
        .try_into()
        .map_err(|e: <u64 as TryFrom<i64>>::Error| ConfigurationError::Error(e.to_string()))?;
    let chunk_size: NonZeroU64 =
        chunk_size
            .try_into()
            .map_err(|e: <NonZeroU64 as TryFrom<u64>>::Error| {
                ConfigurationError::Error(e.to_string())
            })?;

    let chunk_sleep_in_ms: u64 = data["chunk_sleep_in_ms"]
        .as_integer()
        .ok_or(ConfigurationError::new(
            "chunk_sleep_in_ms",
            ErrorType::KeyNotFoundForTable("data"),
        ))?
        .try_into()
        .map_err(|e: <u64 as TryFrom<i64>>::Error| ConfigurationError::Error(e.to_string()))?;
    let chunk_sleep_in_ms: Duration = Duration::from_millis(chunk_sleep_in_ms);

    let dataConf = Data {
        source: source,
        msg_sleep_in_ms,
        chunk_size,
        chunk_sleep_in_ms,
    };

    let config = Config {
        kafka,
        data: dataConf,
    };

    Ok(config.clone())
}
