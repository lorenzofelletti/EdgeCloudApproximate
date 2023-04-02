use self::{errors::ConfigurationError, structs::Config};

mod errors;
mod structs;
mod utils;

pub fn load_config() -> Result<Config, ConfigurationError> {
    Err(todo!())
}
