use std::{error::Error, fs};

use crate::{
    args::{CreateConfig, ReplaceConfig},
    config::{
        constants::{
            DEFAULT_CONSUMER_GROUP, DEFAULT_SAMPLING_STRATEGY, DEFAULT_SEND_EVERY_MS,
            DEFAULT_SEND_STRATEGY, DEFAULT_SOURCE_TOPIC, DEFAULT_TARGET_TOPIC,
            TOML_CONFIG_TEMPLATE,
        },
        structs::Config,
    },
    utils::get_config_path,
};

/// Creates a configuration template in the same directory where the executable
/// is placed into.
pub fn config_create(args: &CreateConfig) -> Result<(), Box<dyn Error>> {
    let output_path = get_config_path()?;

    let source_topic = args
        .source_topic
        .clone()
        .unwrap_or(DEFAULT_SOURCE_TOPIC.to_owned());

    let target_topic = args
        .target_topic
        .clone()
        .unwrap_or(DEFAULT_TARGET_TOPIC.to_owned());

    let toml_template = TOML_CONFIG_TEMPLATE
        .replace("{SOURCE_TOPIC}", &source_topic)
        .replace("{CONSUMER_GROUP}", &DEFAULT_CONSUMER_GROUP.to_string())
        .replace("{TARGET_TOPIC}", &target_topic)
        .replace("{SEND_EVERY_MS}", &DEFAULT_SEND_EVERY_MS.to_string())
        .replace("{SEND_STRATEGY}", &DEFAULT_SEND_STRATEGY)
        .replace("{SAMPLING_STRATEGY}", &DEFAULT_SAMPLING_STRATEGY);

    fs::write(&output_path, toml_template)?;

    Ok(())
}

/// Replaces the current configuration TOML file with a new one.
pub fn config_replace(args: &ReplaceConfig) -> Result<(), Box<dyn Error>> {
    let output_path = get_config_path()?;

    let new_config_contents = fs::read_to_string(args.file.clone())?;

    fs::write(&output_path, new_config_contents)?;

    Ok(())
}

pub fn config_show(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "{:#?}\n{:#?}\n{:#?}",
        config.kafka, config.data_in, config.data_out
    );
    Ok(())
}
