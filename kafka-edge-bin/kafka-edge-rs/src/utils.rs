use std::{env, error::Error, path::PathBuf};

use crate::config::{constants::TOML_FILE_NAME, structs::Config};

/// Returns the Path where the configuration TOML file should be placed.
pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // Get current executable path
    let executable_path = env::current_exe()?;

    let mut output_path = PathBuf::from(executable_path);
    output_path.set_file_name(TOML_FILE_NAME);

    Ok(output_path)
}

/// Return the vector of the topic names to use when in `NeighborhoodWise` strategy.
pub fn get_topics_names_for_neigborhood_wise_strategy(
    config: &Config,
    features: &Vec<geojson::Feature>,
) -> Vec<String> {
    let mut res = vec![];

    for feature in features {
        let neighborhood = feature
            .properties
            .clone()
            .unwrap()
            .get("NAME")
            .expect("NAME property not found in feature")
            .to_string();

        res.push(get_topic_name_from_neighborhood_name(
            &config,
            &neighborhood,
        ));
    }

    res
}

pub fn get_topic_name_from_neighborhood_name(config: &Config, neighborhood: &String) -> String {
    // remove from the neighborhood name the spaces and the quotes
    let neighborhood = neighborhood.replace(" ", "").replace("\"", "");
    format!("{}_{}", config.data_out.target_topic, neighborhood)
}
