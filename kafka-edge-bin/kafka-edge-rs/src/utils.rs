use std::{env, error::Error, path::PathBuf};

use crate::config::{constants::TOML_FILE_NAME, structs::Config};

/// Returns the Path where the configuration TOML file should be placed.
pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // Get current executable path
    let executable_path = env::current_exe()?;

    let mut output_path = executable_path;
    output_path.set_file_name(TOML_FILE_NAME);

    Ok(output_path)
}

/// Return the vector of the topic names to use when in `NeighborhoodWise` strategy.
pub fn get_topics_names_for_neigborhood_wise_strategy(
    config: &Config,
    features: &[geojson::Feature],
) -> Vec<String> {
    let mut res = vec![];

    for (i, _) in features.iter().enumerate() {
        res.push(get_topic_name_from_neighborhood_name(
            config,
            &i.to_string(),
        ));
    }

    // add the topic for the "everything else" neighborhood
    res.push(get_topic_name_from_neighborhood_name(config, "e"));

    res
}

pub fn get_topic_name_from_neighborhood_name(config: &Config, neighborhood: &str) -> String {
    // remove from the neighborhood name the spaces and the quotes
    let neighborhood = neighborhood.replace([' ', '"'], "");
    format!("{}_{}", config.data_out.target_topic, neighborhood)
}

#[macro_export]
macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}
