use std::{
    error::Error,
    process::{Command, ExitStatus},
};

use kafka::client::KafkaClient;

use crate::{args::CreateTopic, config::structs::Config, subcommands::errors::SubcommandError};

use super::utils::join_by_comma;

/// Creates the topic specified in the configuration, with using some optional command line arguments
/// to define some of the topic's characteristics.
/// Under the hood, `kafka-topics.sh` is used, so it will always fail if
/// the script is not available at system level to the user the program
/// is executed with.
pub fn create_topic(config: Config, args: &CreateTopic) -> Result<(), Box<dyn Error>> {
    let replication_factor = args.replication_factor;

    // Connect to Kafka and fetch the metadata for the topic
    let mut client = KafkaClient::new(config.kafka.brokers.clone());
    client.load_metadata_all()?;

    let topic_exists = client
        .topics()
        .names()
        .into_iter()
        .any(|topic| topic.to_owned() == config.kafka.topic);
    if topic_exists {
        println!("Topic already exists!");
        return Ok(());
    }

    let brokers = join_by_comma(&config.kafka.brokers);

    let topic = config.kafka.topic;
    let partitions = config.kafka.partitions;

    // run kafka-topics.sh --create --topic TOPIC --bootstrap-server BROKERS --partiotions PARTITIONS
    // --replication-factor REPLICATION_FACTOR
    let res = Command::new("kafka-topics.sh")
        .arg("--create")
        .arg("--topic")
        .arg(topic)
        .arg("--bootstrap-server")
        .arg(brokers)
        .arg("--partitions")
        .arg(partitions.to_string())
        .arg("--replication-factor")
        .arg(replication_factor.to_string())
        .output()?;

    println!("{}", String::from_utf8_lossy(&res.stdout));

    if ExitStatus::success(&res.status) {
        return Ok(());
    }
    Err(Box::new(SubcommandError::new("Unable to create topic.")))
}
