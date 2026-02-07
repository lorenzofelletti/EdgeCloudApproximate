use std::{error::Error, path::PathBuf};

use crate::{
    args::Neighborhoods,
    config::structs::Config,
    geospatial::read_neighborhoods,
    utils::{get_neighborhoods_names, get_topics_names_for_neigborhood_wise_strategy},
};

pub fn show_neighborhoods(
    config: Option<Config>,
    args: &Neighborhoods,
) -> Result<(), Box<dyn Error>> {
    let file = match &args.file_path {
        Some(file) => Ok(PathBuf::from(file)),
        None => {
            if let Some(config) = config.clone() {
                Ok(config.data_out.neighborhoods_file.clone())
            } else {
                Err(format!("no config found and no file path provided"))
            }
        }
    }?;

    let neighborhoods = read_neighborhoods(&file)?;

    if args.show_generated_topic_names {
        if config.clone().is_none() {
            return Err(format!(
                "option --show-generated-topic-names requires a config file, but none was found."
            )
            .into());
        }
        let config = config.unwrap();

        let topics = get_topics_names_for_neigborhood_wise_strategy(&config, &neighborhoods);
        for topic in topics {
            println!("{}", topic);
        }
    } else {
        let name_property = args.name_property.clone().unwrap_or("name".to_string());
        let names = get_neighborhoods_names(&neighborhoods, name_property)?;
        for name in names {
            println!("{}", name);
        }
    }

    Ok(())
}
