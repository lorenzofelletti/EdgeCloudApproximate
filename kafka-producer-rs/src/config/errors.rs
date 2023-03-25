use std::{
    error::Error,
    fmt::{self},
};
/*
#[derive(Debug)]
pub struct ConfigError {
    pub message: String,
}

impl ConfigError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing the configuration: {}", self.message)
    }
}*/

#[derive(Debug)]
pub enum ErrorType<S: Into<String>> {
    TableNotFound,
    KeyNotFoundForTable(S),
    InvalidValueForKey(S),
    Error,
}

#[derive(Debug)]
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
}
