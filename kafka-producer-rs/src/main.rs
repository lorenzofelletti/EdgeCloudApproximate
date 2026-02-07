use std::error::Error;

use args::{CliArgs, EditConfigCommands};
use clap::Parser;
use config::load_config;

use kafka_producer::run_kafka_producer;
use subcommands::{
    create_topic::create_topic,
    delete_topic::delete_topic,
    edit_config::{edit_config_create, edit_config_replace},
};

use crate::kafka_producer::{message, message::Message};

mod args;
mod config;
mod kafka_producer;
mod subcommands;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config();

    let cli = CliArgs::parse();

    match &cli.subcommands {
        Some(args::Commands::CreateTopic(args)) => {
            create_topic(config?.clone(), args)?;
        }
        Some(args::Commands::DeleteTopic) => {
            delete_topic(config?.clone())?;
        }
        Some(args::Commands::EditConfig(args)) => match &args.subcommands {
            EditConfigCommands::Create => {
                edit_config_create()?;
            }
            EditConfigCommands::Replace(args) => {
                edit_config_replace(args)?;
            }
        },
        None => {
            let config = config?;
            match config.data.message_type {
                kafka_producer::message::MessageType::Message => {
                    run_kafka_producer::<Message>(config, &cli)?;
                }
                kafka_producer::message::MessageType::Value => {
                    let config_key = config
                        .data
                        .key
                        .clone()
                        .ok_or("key required for value messages, but not found in config")?;

                    {
                        // sets the key, and release the lock
                        let mut key = message::KEY.write()?;
                        *key = config_key;
                    }

                    run_kafka_producer::<serde_json::Value>(config, &cli)?;
                }
            }
        }
    }

    Ok(())
}
