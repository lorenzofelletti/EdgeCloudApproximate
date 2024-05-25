use std::num::NonZeroU64;

use toml::{map::Map, Value};

use super::errors::{ConfigurationError, ErrorType};

#[inline]
pub fn vec_empty_or_has_empty_strings(to_check: &[String]) -> bool {
    to_check.is_empty() || to_check.contains(&String::from(""))
}

/// Checks that the value for `key` is not an empty vec and that it does not contains empty strings
pub fn check_value_not_empty_or_has_empty_strings<S: Into<String>>(
    value: &[String],
    key_name: S,
) -> Result<(), ConfigurationError> {
    if vec_empty_or_has_empty_strings(value) {
        return Err(ConfigurationError::new(
            "",
            ErrorType::InvalidValueForKey(&key_name.into()),
        ));
    }
    Ok(())
}

/// Returns the table from a configuration file, if it exists.
pub fn get_table<S: Into<String>>(
    table_name: S,
    config: &Value,
) -> Result<Map<String, Value>, ConfigurationError> {
    let table_name: String = table_name.into();
    config[table_name]
        .as_table()
        .cloned()
        .ok_or(ConfigurationError::new("kafka", ErrorType::TableNotFound))
}

/// Reads an integer key from a table
pub fn read_integer_key_from_table<S: Into<String>>(
    table_name: S,
    key_name: S,
    data: &toml::map::Map<std::string::String, Value>,
) -> Result<i64, ConfigurationError> {
    let key_name: String = key_name.into();
    data[&key_name.clone()]
        .as_integer()
        .ok_or(ConfigurationError::new_key_not_found_err(
            key_name,
            table_name.into(),
        ))
}

pub fn read_array_key_from_table<S: Into<String>>(
    table_name: S,
    key_name: S,
    table_data: &toml::map::Map<std::string::String, Value>,
) -> Result<&Vec<Value>, ConfigurationError> {
    let key_name: String = key_name.into();
    if !table_data.contains_key(&key_name) {
        return Err(ConfigurationError::new_key_not_found_err(
            key_name,
            table_name.into(),
        ));
    }
    table_data[&key_name.clone()]
        .as_array()
        .ok_or(ConfigurationError::new_key_not_found_err(
            key_name,
            table_name.into(),
        ))
}

/// Returns Ok if the key exists in the specified table, an Err otherwise.
fn exists_key<S: Into<String>, S2: Into<String>>(
    table_name: S,
    key_name: S2,
    table_data: &toml::map::Map<std::string::String, Value>,
) -> Result<(), ConfigurationError> {
    let key_name: String = key_name.into();
    match table_data.contains_key(&key_name) {
        true => Ok(()),
        false => Err(ConfigurationError::new_key_not_found_err(
            key_name,
            table_name.into(),
        )),
    }
}

pub fn read_string_key_from_table<S: Into<String>>(
    table_name: S,
    key_name: S,
    table_data: &toml::map::Map<std::string::String, Value>,
) -> Result<String, ConfigurationError> {
    let key_name: String = key_name.into();

    exists_key(table_name, key_name.clone(), table_data)?;

    let str = table_data[&key_name.clone()]
        .as_str()
        .ok_or(ConfigurationError::new(
            table_data[&key_name.clone()].to_string(),
            ErrorType::InvalidValueForKey(key_name),
        ))?;
    Ok(str.to_owned())
}

/// Converts i64 to u64, returning a `ConfigurationError` if the conversion fails
#[allow(dead_code)]
pub fn from_i64_to_u64(value: i64) -> Result<u64, ConfigurationError> {
    let res: u64 = value
        .try_into()
        .map_err(|e: <u64 as TryFrom<i64>>::Error| ConfigurationError::Error(e.to_string()))?;
    Ok(res)
}

#[allow(dead_code)]
pub fn from_u64_to_nonzerou64(value: u64) -> Result<NonZeroU64, ConfigurationError> {
    value
        .try_into()
        .map_err(|e: <NonZeroU64 as TryFrom<u64>>::Error| ConfigurationError::Error(e.to_string()))
}

pub fn from_vec_of_value_to_vec_of_string(vec_of_value: &[Value]) -> Vec<String> {
    vec_of_value
        .iter()
        .map(|x| x.as_str().unwrap_or_default().to_owned())
        .collect()
}

pub fn from_vec_of_string_to_vec_of_i32(vec_of_string: &[String]) -> Vec<i32> {
    vec_of_string
        .iter()
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect()
}
