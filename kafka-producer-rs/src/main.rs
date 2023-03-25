use std::{thread::sleep, time::Duration};

use args::{CliArgs, EditConfigCommands};
use clap::Parser;
use config::load_config;
use kafka::producer::{Producer, RequiredAcks};
use kafka_producer::{errors::KafkaSendError, message::Message};
use subcommands::{
    delete_topic::delete_topic,
    edit_config::{self, edit_config_create},
};

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
    let config = load_config();

    let cli = CliArgs::parse();

    match &cli.subcommands {
        Some(args::Commands::DeleteTopic) => {
            let config = config?;
            delete_topic(config.clone())?;
        }
        Some(args::Commands::EditConfig(args)) => match &args.subcommands {
            Some(EditConfigCommands::Create) => {
                edit_config_create()?;
            }
            Some(EditConfigCommands::Replace(args)) => {
                let config = config?;
            }
            None => {
                let config = config?;
            }
        },
        None => {
            let config = config?;
        }
    }

    Ok(())
}
