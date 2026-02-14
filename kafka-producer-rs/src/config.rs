use std::{
    fs,
    num::NonZeroU64,
    path::{Path, PathBuf},
    time::Duration,
};

use toml::Value;

use crate::utils::get_config_path;

use self::{
    errors::{ConfigurationError, ErrorType},
    structs::{Config, Data, Kafka},
    utils::{
        check_value_not_empty_or_has_empty_strings, from_i64_to_u64, from_u64_to_nonzerou64,
        from_vec_of_value_to_vec_of_string, get_table, read_array_key_from_table,
        read_integer_key_from_table,
    },
};

pub mod errors;
pub mod structs;
mod utils;

/// Parses the 'kafka' table of the configuration.
fn parse_kafka_table(config: &Value) -> Result<Kafka, ConfigurationError> {
    let kafka = get_table("kafka", config)?;

    let zookeeper = read_array_key_from_table("kafka", "zookeeper", &kafka)?;
    let zookeeper: Vec<String> = from_vec_of_value_to_vec_of_string(zookeeper);
    check_value_not_empty_or_has_empty_strings(&zookeeper, "zookeeper")?;

    let brokers = read_array_key_from_table("kafka", "brokers", &kafka)?;
    let brokers: Vec<String> = from_vec_of_value_to_vec_of_string(brokers);
    check_value_not_empty_or_has_empty_strings(&brokers, "brokers")?;

    let topic = kafka["topic"]
        .as_str()
        .ok_or(ConfigurationError::new(
            "topic",
            ErrorType::KeyNotFoundForTable("kafka"),
        ))?
        .to_owned();

    let partitions = read_integer_key_from_table("kafka", "partitions", &kafka)?;
    let partitions = from_i64_to_u64(partitions).and_then(from_u64_to_nonzerou64)?;

    Ok(Kafka {
        zookeeper,
        brokers,
        topic,
        partitions,
    })
}

/// Parses the 'data' table of the configuration file.
fn parse_data_table(config: &Value) -> Result<Data, ConfigurationError> {
    let data = get_table("data", &config)?;

    let source = data["source"].as_str().ok_or(ConfigurationError::new(
        "source",
        ErrorType::KeyNotFoundForTable("data"),
    ))?;
    let source = PathBuf::from(source);

    let msg_sleep_in_ms = read_integer_key_from_table("data", "msg_sleep_in_ms", &data)?;
    let msg_sleep_in_ms: u64 = from_i64_to_u64(msg_sleep_in_ms)?;
    let msg_sleep_in_ms = Duration::from_millis(msg_sleep_in_ms);

    let chunk_size = read_integer_key_from_table("data", "chunk_size", &data)?;
    let chunk_size: NonZeroU64 = from_i64_to_u64(chunk_size).and_then(from_u64_to_nonzerou64)?;

    let chunk_sleep_in_ms = read_integer_key_from_table("data", "chunk_sleep_in_ms", &data)?;
    let chunk_sleep_in_ms: u64 = from_i64_to_u64(chunk_sleep_in_ms)?;
    let chunk_sleep_in_ms: Duration = Duration::from_millis(chunk_sleep_in_ms);
    Ok(Data {
        source,
        msg_sleep_in_ms,
        chunk_size,
        chunk_sleep_in_ms,
    })
}

fn load_config_from(file: &Path) -> Result<Config, ConfigurationError> {
    let contents = fs::read_to_string(file)
        .map_err(|_| ConfigurationError::new("Configuration file not found.", ErrorType::Error))?;

    let config: Value = toml::from_str(&contents)
        .map_err(|e| ConfigurationError::new(e.message(), ErrorType::Error))?;

    // Parses 'kakfa' table
    let kafka = parse_kafka_table(&config)?;

    // Parses 'data' table
    let data = parse_data_table(&config)?;

    Ok(Config { kafka, data })
}

/// Tries to load the program's configuration TOML file.
pub fn load_config() -> Result<Config, ConfigurationError> {
    let path = get_config_path()
        .map_err(|_| ConfigurationError::new("Configuration file not found.", ErrorType::Error))?;
    load_config_from(path.as_ref())
}
