use std::{error::Error, fs};

use crate::args::ReplaceConfig;

use super::{toml_template::TOML_CONFIG_TEMPLATE, utils::get_config_path};

pub fn edit_config_create() -> Result<(), Box<dyn Error>> {
    let output_path = get_config_path()?;

    let data_source = format!("/home/{}/data.csv", whoami::username());

    let toml_template = TOML_CONFIG_TEMPLATE.replace("{data_source}", &data_source);

    fs::write(&output_path, toml_template)?;

    Ok(())
}

pub fn edit_config_replace(args: &ReplaceConfig) -> Result<(), Box<dyn Error>> {
    let output_path = get_config_path()?;

    let new_config_contents = fs::read_to_string(args.file.clone())?;

    fs::write(&output_path, new_config_contents)?;

    Ok(())
}
