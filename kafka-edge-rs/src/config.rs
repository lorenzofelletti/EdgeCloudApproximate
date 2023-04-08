use std::{fs, path::Path, time::Duration};

use toml::Value;

use crate::{
    kafka_producer::strategies::{SamplingStrategy, SendStrategy},
    utils::get_config_path,
};

use self::{
    errors::{ConfigurationError, ErrorType},
    structs::{Config, DataIn, DataOut, Kafka},
    utils::{
        check_value_not_empty_or_has_empty_strings, from_i64_to_u64,
        from_vec_of_value_to_vec_of_string, get_table, read_array_key_from_table,
        read_integer_key_from_table, read_string_key_from_table,
    },
};

#[macro_use] mod macros;
pub mod constants;
mod errors;
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

    Ok(Kafka { zookeeper, brokers })
}

/// Parses the 'data' table of the configuration file.
fn parse_data_in_table(config: &Value) -> Result<DataIn, ConfigurationError> {
    let data = get_table("data_in", &config)?;

    let source_topic = read_string_key_from_table("data_in", "source_topic", &data)?;

    let partition_to_read = read_integer_key_from_table("data_in", "partition_to_read", &data)?;
    let partition_to_read: i32 = partition_to_read
        .try_into()
        .expect("Partition number too big!");

    Ok(DataIn {
        source_topic,
        partition_to_read,
    })
}

/// Parses the 'data' table of the configuration file.
fn parse_data_out_table(config: &Value) -> Result<DataOut, ConfigurationError> {
    let data = get_table("data_out", &config)?;

    let target_topic = read_string_key_from_table("data_out", "target_topic", &data)?;

    let send_every_ms = read_integer_key_from_table("data_out", "send_every_ms", &data)?;
    let send_every_ms: u64 = from_i64_to_u64(send_every_ms)?;
    let send_every_ms = Duration::from_millis(send_every_ms);

    let send_strategy = read_string_key_from_table("data_out", "send_strategy", &data)?;
    let send_strategy = match send_strategy.to_lowercase().as_str() {
        "roundrobin" => Ok(SendStrategy::RoundRobin),
        "random" => Ok(SendStrategy::Random),
        _ => Err(ConfigurationError::new(
            send_strategy,
            ErrorType::InvalidValueForKey("send_strategy".to_owned()),
        )),
    }?;

    let sampling_strategy = read_string_key_from_table("data_out", "sampling_strategy", &data)?;
    let sampling_strategy = match sampling_strategy.to_lowercase().as_str() {
        "stratified" => Ok(SamplingStrategy::Stratified),
        "random" => Ok(SamplingStrategy::Random),
        _ => Err(ConfigurationError::new(
            sampling_strategy,
            ErrorType::InvalidValueForKey("sampling_strategy".to_owned()),
        )),
    }?;

    Ok(DataOut {
        target_topic,
        send_every_ms,
        send_strategy,
        sampling_strategy,
    })
}

fn load_config_from(file: &Path) -> Result<Config, ConfigurationError> {
    // Reads the content of the configuration file
    let contents = fs::read_to_string(file)
        .map_err(|_| ConfigurationError::new("Configuration file not found.", ErrorType::Error))?;

    // Deserializes contents
    let config: Value = toml::from_str(&contents)
        .map_err(|e| ConfigurationError::new(e.message(), ErrorType::Error))?;

    let kafka = parse_kafka_table(&config)?;

    let data_in = parse_data_in_table(&config)?;

    let data_out = parse_data_out_table(&config)?;

    Ok(Config {
        kafka,
        data_in,
        data_out,
    })
}

pub fn load_config() -> Result<Config, ConfigurationError> {
    let path = get_config_path()
        .map_err(|_| ConfigurationError::new("Configuration file not found.", ErrorType::Error))?;
    //load_config_from(path.as_ref())
    Err(todo!())
}
