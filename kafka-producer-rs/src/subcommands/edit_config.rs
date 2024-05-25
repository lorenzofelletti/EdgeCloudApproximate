use std::{error::Error, fs};

use crate::{args::ReplaceConfig, utils::get_config_path};

use super::toml_template::TOML_CONFIG_TEMPLATE;

/// Creates a configuration template in the same directory where the executable
/// is placed into.
pub fn edit_config_create() -> Result<(), Box<dyn Error>> {
    let output_path = get_config_path()?;

    let data_source = format!("/home/{}/data.csv", whoami::username());

    let toml_template = TOML_CONFIG_TEMPLATE.replace("{data_source}", &data_source);

    fs::write(output_path, toml_template)?;

    Ok(())
}

/// Replaces the current configuration TOML file with a new one.
pub fn edit_config_replace(args: &ReplaceConfig) -> Result<(), Box<dyn Error>> {
    let output_path = get_config_path()?;

    let new_config_contents = fs::read_to_string(args.file.clone())?;

    fs::write(output_path, new_config_contents)?;

    Ok(())
}
