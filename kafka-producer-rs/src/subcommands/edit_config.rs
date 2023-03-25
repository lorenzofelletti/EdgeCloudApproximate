use std::{env, error::Error, fs, path::PathBuf};

use super::toml_template::TOML_CONFIG_TEMPLATE;

pub fn edit_config_create() -> Result<(), Box<dyn Error>> {
    let executable_path = env::current_exe()?;

    let data_source = format!("/home/{}/data.csv", whoami::username());

    let mut output_path = PathBuf::from(executable_path);
    output_path.set_file_name("producer_config.toml");

    let toml_template = TOML_CONFIG_TEMPLATE.replace("{data_source}", &data_source);

    fs::write(&output_path, toml_template)?;

    Ok(())
}
