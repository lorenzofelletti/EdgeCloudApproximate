use std::{
    error::Error,
    process::{Command, ExitStatus},
};

use kafka::client::KafkaClient;

use crate::{config::structs::Config, subcommands::errors::SubcommandError};

use super::utils::get_zookeeper_string;

pub fn delete_topic(config: Config) -> Result<(), Box<dyn Error>> {
    // Connect to Kafka and fetch the metadata for the topic
    let mut client = KafkaClient::new(config.kafka.brokers);
    client.load_metadata_all()?;

    let topic_exists = client
        .topics()
        .names()
        .into_iter()
        .any(|topic| topic.to_owned() == config.kafka.topic);
    if !topic_exists {
        println!("Topic does not exist!");
        return Ok(());
    }

    let zookeeper = get_zookeeper_string(&config.kafka.zookeeper);

    let topic = config.kafka.topic;

    // run kafka-topics.sh --zookeeper ZOOKEEPER --delete --topic TOPIC
    let res = Command::new("kafka-topics.sh")
        .arg("--zookeeper")
        .arg(zookeeper)
        .arg("--delete")
        .arg("--topic")
        .arg(topic)
        .output()?;

    println!("{}", String::from_utf8_lossy(&res.stdout));

    if ExitStatus::success(&res.status) {
        return Ok(());
    }
    Err(Box::new(SubcommandError::new("Unable to delete topic.")))
}
