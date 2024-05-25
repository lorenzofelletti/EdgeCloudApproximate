use std::{env, error::Error, path::PathBuf};

use crate::subcommands::toml_template::TOML_FILE_NAME;

/// Returns the Path where the configuration TOML file should be placed.
pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // Get current executable path
    let mut executable_path = env::current_exe()?;

    executable_path.set_file_name(TOML_FILE_NAME);

    Ok(executable_path)
}
