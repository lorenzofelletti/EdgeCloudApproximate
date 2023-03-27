use args::{CliArgs, EditConfigCommands};
use clap::Parser;
use config::load_config;

use kafka_producer::run_kafka_producer;
use subcommands::{
    create_topic::create_topic,
    delete_topic::delete_topic,
    edit_config::{edit_config_create, edit_config_replace},
};

mod args;
mod config;
mod kafka_producer;
mod subcommands;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            run_kafka_producer(config?.clone(), &cli)?;
        }
    }

    Ok(())
}
