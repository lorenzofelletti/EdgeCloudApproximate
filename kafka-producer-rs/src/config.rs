use std::{fs, num::NonZeroU64, path::Path, time::Duration};

use toml::{map::Map, Value};

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

/// Checks that the 'brokers' value of the configuration is valid
fn check_brokers(brokers: &Vec<String>) -> Result<(), ConfigurationError> {
    if brokers.len() == 0 || brokers.contains(&String::from("")) {
        return Err(ConfigurationError::new(
            "",
            ErrorType::InvalidValueForKey("brokers"),
        ));
    }
    Ok(())
}

/// Returns the table from a configuration file, if it exists.
fn get_table<'a, S: Into<String>>(
    table_name: S,
    config: &'a Value,
) -> Result<&'a Map<String, Value>, ConfigurationError> {
    let table_name: String = table_name.into();
    config[table_name]
        .as_table()
        .ok_or(ConfigurationError::new("kafka", ErrorType::TableNotFound))
}

/// Reads an integer key from a table
fn read_integer_key_from_table<S: Into<String>>(
    table_name: S,
    key_name: S,
    data: &toml::map::Map<std::string::String, Value>,
) -> Result<i64, ConfigurationError> {
    let key_name: String = key_name.into();
    data[&key_name.clone()]
        .as_integer()
        .ok_or(ConfigurationError::new(
            key_name.clone(),
            ErrorType::KeyNotFoundForTable(table_name.into()),
        ))
}

/// Converts i64 to u64, returning a `ConfigurationError` if the conversion fails
fn from_i64_to_u64(value: i64) -> Result<u64, ConfigurationError> {
    let res: u64 = value
        .try_into()
        .map_err(|e: <u64 as TryFrom<i64>>::Error| ConfigurationError::Error(e.to_string()))?;
    Ok(res)
}

pub fn load_config(from: &Path) -> Result<Config<'static>, ConfigurationError> {
    let contents =
        fs::read_to_string(from).expect("Config file `producer_config.toml` should exists!");

    let config: Value = toml::from_str(&contents)
        .map_err(|e| ConfigurationError::new(e.message(), ErrorType::Error))?;

    // Parses 'kakfa' table
    let kafka = get_table("kafka", &config)?;

    let brokers = kafka["brokers"].as_array().ok_or(ConfigurationError::new(
        "brokers",
        ErrorType::KeyNotFoundForTable("kafka"),
    ))?;
    let brokers: Vec<String> = brokers
        .into_iter()
        .map(|x| x.as_str().unwrap_or_default().to_owned())
        .collect();

    check_brokers(&brokers)?;

    let kafka = Kafka { brokers };

    // Parses 'data' table
    let data = get_table("data", &config)?;

    let source = data["source"].as_str().ok_or(ConfigurationError::new(
        "source",
        ErrorType::KeyNotFoundForTable("data"),
    ))?;
    let source = Path::new(source);

    let msg_sleep_in_ms = read_integer_key_from_table("data", "msg_sleep_in_ms", data)?;
    let msg_sleep_in_ms: u64 = from_i64_to_u64(msg_sleep_in_ms)?;
    let msg_sleep_in_ms = Duration::from_millis(msg_sleep_in_ms);

    let chunk_size = read_integer_key_from_table("data", "chunk_size", data)?;
    let chunk_size: u64 = from_i64_to_u64(chunk_size)?;
    let chunk_size: NonZeroU64 =
        chunk_size
            .try_into()
            .map_err(|e: <NonZeroU64 as TryFrom<u64>>::Error| {
                ConfigurationError::Error(e.to_string())
            })?;

    let chunk_sleep_in_ms = read_integer_key_from_table("data", "chunk_sleep_in_ms", data)?;
    let chunk_sleep_in_ms: u64 = from_i64_to_u64(chunk_sleep_in_ms)?;
    let chunk_sleep_in_ms: Duration = Duration::from_millis(chunk_sleep_in_ms);

    let source = source.as_ref();
    let data: Data = Data {
        source,
        msg_sleep_in_ms,
        chunk_size,
        chunk_sleep_in_ms,
    };

    Ok(Config { kafka, data })
}
