use std::{fs, path::Path, time::Duration};

use config_proc_macros::table_name;
use toml::Value;

use crate::{
    kafka_producer::strategies::{SamplingStrategy, SendStrategy},
    utils::get_config_path,
};

use self::{
    errors::{ConfigurationError, ErrorType},
    structs::{Config, DataIn, DataOut, Kafka},
    utils::{
        check_value_not_empty_or_has_empty_strings, from_vec_of_value_to_vec_of_string, get_table,
        read_array_key_from_table, read_integer_key_from_table, read_string_key_from_table,
    },
};

#[macro_use]
mod macros;
pub mod constants;
mod errors;
pub mod structs;
mod utils;

/// Parses the 'kafka' table of the configuration.
#[table_name(kafka)]
fn parse_kafka_table(config: &Value) -> Result<Kafka, ConfigurationError> {
    read_array_of_hosts!(zookeeper, table_name, data);
    read_array_of_hosts!(brokers, table_name, data);

    Ok(Kafka { zookeeper, brokers })
}

/// Parses the 'data' table of the configuration file.
#[table_name(data_in)]
fn parse_data_in_table(config: &Value) -> Result<DataIn, ConfigurationError> {
    let source_topic = read_string_key_from_table(table_name, "source_topic", &data)?;

    let partition_to_read = read_integer_key_from_table(table_name, "partition_to_read", &data)?;
    let partition_to_read: i32 = partition_to_read
        .try_into()
        .expect("Partition number too big!");

    Ok(DataIn {
        source_topic,
        partition_to_read,
    })
}

/// Parses the 'data' table of the configuration file.
#[table_name(data_out)]
fn parse_data_out_table(config: &Value) -> Result<DataOut, ConfigurationError> {
    read_duration!(send_every_ms, table_name, data);

    let target_topic = read_string_key_from_table(table_name, "target_topic", &data)?;

    read_string_with_match! {
        send_strategy,
        table_name,
        data,
        "roundrobin" => Ok(SendStrategy::RoundRobin),
        "random" => Ok(SendStrategy::Random)
    };

    read_string_with_match! {
        sampling_strategy,
        table_name,
        data,
        "stratified" => Ok(SamplingStrategy::Stratified),
        "random" => Ok(SamplingStrategy::Random)
    };

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
        .map_err(|_| ConfigurationError::new("Unexpected error.", ErrorType::Error))?;
    match path.exists() {
        true => load_config_from(path.as_ref()),
        false => Err(ConfigurationError::new(
            "Configuration file does not exists!",
            ErrorType::Error,
        )),
    }
}
