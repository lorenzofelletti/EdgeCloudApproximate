use std::{path::Path, thread::sleep, time::Duration};

use args::CliArgs;
use clap::Parser;
use config::load_config;
use kafka::producer::{Producer, RequiredAcks};
use kafka_producer::{errors::KafkaSendError, message::Message};

mod args;
mod config;
mod kafka_producer;
mod subcommands;

fn send_to_kafka_topic(
    msg: Message,
    topic: String,
    partition: Option<String>,
    brokers: Vec<String>,
) -> Result<(), KafkaSendError> {
    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create();
    Err(KafkaSendError::new_unknown_error(
        "Error sending the message",
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let cli = CliArgs::parse();

    match &cli.subcommands {
        Some(args::Commands::DeleteTopic) => {}
        Some(args::Commands::EditConfig(args)) => {}
        None => {}
    }

    let mut reader = csv::Reader::from_path(config.data.source).unwrap();

    for row in reader.records() {
        match row {
            Ok(row) => {
                let data: Message = row.deserialize(None)?;
                //let brokers = brokers.into_iter().map(|x| x.to_owned()).collect();
                //send_to_kafka_topic(msg, topic, partition, brokers);
                if !config.data.msg_sleep_in_ms.is_zero() {
                    sleep(config.data.msg_sleep_in_ms);
                };
            }
            Err(e) => {
                println!("Error reading row: `{e}'");
                continue;
            }
        }
    }

    Ok(())
}
