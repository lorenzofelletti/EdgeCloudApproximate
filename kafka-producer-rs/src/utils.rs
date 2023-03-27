use std::{env, error::Error, path::PathBuf};

use crate::subcommands::toml_template::TOML_FILE_NAME;

/// Returns the Path where the configuration TOML file should be placed.
pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // Get current executable path
    let executable_path = env::current_exe()?;

    let mut output_path = PathBuf::from(executable_path);
    output_path.set_file_name(TOML_FILE_NAME);

    Ok(output_path)
}
