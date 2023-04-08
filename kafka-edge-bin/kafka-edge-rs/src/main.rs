use args::{CliArgs, EditConfigCommands, TopicCommands};
use clap::Parser;
use config::load_config;
use kafka_producer::run_producer;
use subcommands::{
    config::{config_create, config_replace, config_show},
    topic::topic_create,
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
        Some(args::Commands::EditConfig(edit)) => match &edit.subcommands {
            EditConfigCommands::Create(args) => config_create(args),
            EditConfigCommands::Replace(args) => config_replace(args),
            EditConfigCommands::Show => config_show(config?),
        },
        Some(args::Commands::Topic(topic)) => {
            let config = config?;
            match &topic.subcommands {
                TopicCommands::Create(args) => topic_create(config, args),
                TopicCommands::Delete(_args) => todo!(),
            }
        }
        None => run_producer(config?, &cli),
    }
}
