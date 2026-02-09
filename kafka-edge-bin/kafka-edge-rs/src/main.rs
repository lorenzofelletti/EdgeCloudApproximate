use std::error::Error;

use args::{CliArgs, EditConfigCommands, TopicCommands};
use clap::Parser;
use config::load_config;
use kafka_producer::run_producer;
use subcommands::{
    config::{config_create, config_replace, config_show},
    topic::{topic_create, topic_delete},
};

use crate::{kafka_producer::message::Message, subcommands::geojson::show_neighborhoods};

mod args;
mod config;
mod csv_producer;
mod geospatial;
mod kafka_producer;
mod subcommands;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

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
                TopicCommands::Create(args) => topic_create(&config, args),
                TopicCommands::Delete(args) => topic_delete(config, args),
            }
        }
        Some(args::Commands::Geojson(geojson)) => match &geojson.subcommands {
            args::GeojsonCommands::Neighborhoods(neighborhoods) => {
                let config = config.ok();
                show_neighborhoods(config, neighborhoods)
            }
        },
        Some(args::Commands::CSVProducer(producer)) => {
            match (producer.input_file.clone(), producer.raw_json_messages) {
                (None, true) => csv_producer::run_producer::<
                    serde_json::Map<String, serde_json::Value>,
                >(config?, &producer),
                (None, false) => csv_producer::run_producer::<Message>(config?, &producer),
                (Some(_), true) => csv_producer::run_producer_from_file::<
                    serde_json::Map<String, serde_json::Value>,
                >(config?, &producer),
                (Some(_), false) => {
                    csv_producer::run_producer_from_file::<Message>(config?, &producer)
                }
            }
        }
        None => run_producer(config?, &cli),
    }
}
