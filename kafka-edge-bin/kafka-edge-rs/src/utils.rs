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

    for i in 1..=features.len() {
        res.push(format!("{}{i}", config.data_out.target_topic))
    }

    res
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::kafka_producer::strategies::SamplingStrategy;

    use super::*;

    fn generate_mock_conf() -> Config {
        use crate::config::structs::{DataIn, DataOut, Kafka};
        Config {
            kafka: Kafka {
                zookeeper: vec!["".to_owned()],
                brokers: vec!["".to_owned()],
            },
            data_in: DataIn {
                source_topic: "source".to_owned(),
                consumer_group: "cons".to_owned(),
            },
            data_out: DataOut {
                target_topic: "target".to_owned(),
                send_every_ms: Duration::from_millis(1),
                send_strategy: crate::kafka_producer::strategies::SendStrategy::NeighborhoodWise,
                neighborhoods_file: Some(PathBuf::from("")),
                sampling_strategy: SamplingStrategy::Random,
            },
        }
    }

    fn generate_mock_features_vec() -> Vec<geojson::Feature> {
        vec![
            geojson::Feature::default(),
            geojson::Feature::default(),
            geojson::Feature::default(),
        ]
    }

    #[test]
    pub fn test_get_topics_names_for_neigborhood_wise_strategy() {
        let cfg = generate_mock_conf();
        let features = generate_mock_features_vec();
        let topics_names = get_topics_names_for_neigborhood_wise_strategy(&cfg, &features);

        assert_eq!(3, topics_names.len());
        for i in 0..topics_names.len() {
            assert_eq!(
                format!("{}{}", cfg.data_out.target_topic, i + 1),
                topics_names[i]
            );
        }
    }
}
