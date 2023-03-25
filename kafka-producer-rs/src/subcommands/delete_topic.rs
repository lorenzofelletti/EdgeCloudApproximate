use std::{error::Error, process::Command};

use kafka::client::KafkaClient;

use crate::config::structs::Config;

pub fn delete_topic(config: Config) -> Result<(), Box<dyn Error>> {
    // Connect to Kafka and fetch the metadata for the topic
    let mut client = KafkaClient::new(config.kafka.brokers);
    client.load_metadata_all()?;

    let topic_exists = client.topics().names().into_iter().any(|topic| topic.to_owned() == config.kafka.topic);
    if !topic_exists {
        println!("Topic does not exist!");
        return Ok(());
    }

    //let zookeeper = 

    //let output = Command::new("").arg("").arg("--delete").output();

    // run kafka-topics.sh --zookeeper zookeeper:2181 --delete --topic test-topic

    Ok(())
}
