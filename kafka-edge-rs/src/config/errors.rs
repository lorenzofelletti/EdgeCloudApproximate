use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug, Clone, Copy)]
/// Enum of the possible errors that may happen when loading the configuration TOML file.
pub enum ErrorType<S: Into<String>> {
    TableNotFound,
    /// `KeyNotFoundForTable(table)`
    KeyNotFoundForTable(S),
    /// `InvalidValueForKey(key)`
    InvalidValueForKey(S),
    Error,
}

#[derive(Debug, Clone)]
/// Enum of different errors that may occur when loading the configuration TOML file.
pub enum ConfigurationError {
    TableNotFound(String),
    /// `KeyNotFound(table, key)`
    KeyNotFound(String, String),
    /// `InvalidValue(key, value)`
    InvalidValue(String, String),
    Error(String),
}

impl Error for ConfigurationError {}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationError::TableNotFound(t) => write!(f, "No table '{t}' found."),
            ConfigurationError::KeyNotFound(t, k) => {
                write!(f, "No key '{k}' found in table '{t}'.")
            }
            ConfigurationError::InvalidValue(k, v) => {
                write!(f, "Invalid value `{v}` for key '{k}'.")
            }
            ConfigurationError::Error(e) => write!(f, "{e}"),
        }
    }
}

impl ConfigurationError {
    /// Creates a new `ConfigurationError` of the specified `ErrorType`.
    pub fn new<S: Into<String>>(value: S, error_type: ErrorType<S>) -> Self {
        match error_type {
            ErrorType::TableNotFound => ConfigurationError::TableNotFound(value.into()),
            ErrorType::KeyNotFoundForTable(t) => {
                ConfigurationError::KeyNotFound(t.into(), value.into())
            }
            ErrorType::InvalidValueForKey(k) => {
                ConfigurationError::InvalidValue(k.into(), value.into())
            }
            ErrorType::Error => ConfigurationError::Error(value.into()),
        }
    }

    pub fn new_key_not_found_err<S: Into<String>>(key: S, table: S) -> Self {
        ConfigurationError::KeyNotFound(key.into(), table.into())
    }
}
