use std::{num::NonZeroU32, path::PathBuf};

use clap::{Args, Parser, Subcommand};

const ABOUT: &str = "Kafka Edge Producer \n
Reads from a Kafka IN topic data, samples it, and then sends it to a Kafka OUT topic. \n
It uses a TOML configuration file, placed in the same directory as the executable, \
for configuration.";

#[derive(Parser)]
#[command(author, version, about = ABOUT, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[arg(short, long, default_value_t = 0.5)]
    /// The sampling percentage; i.e. how many of the incoming messages to
    /// discard, in percentage
    pub sampling_percentage: f64,

    #[arg(long)]
    /// Override the strategy indicated in the TOML configuration file
    pub override_send_strategy: Option<String>,

    #[command(subcommand)]
    pub subcommands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "topic")]
    /// Manage topics
    Topic(Topic),

    #[command(name = "config")]
    /// Create or edit configuration
    EditConfig(EditConfig),
}

#[derive(Args)]
pub struct Topic {
    #[command(subcommand)]
    pub subcommands: TopicCommands,
}

#[derive(Subcommand)]
pub enum TopicCommands {
    Create(TopicCreateArgs),
    Delete(TopicDeleteArgs),
}

#[derive(Args)]
pub struct EditConfig {
    #[command(subcommand)]
    pub subcommands: EditConfigCommands,
}

#[derive(Args)]
pub struct TopicCreateArgs {
    #[arg()]
    /// Kafka topic to create ("out" only supported at the moment)
    pub topic: String,
    #[arg(long, default_value_t = NonZeroU32::new(1).unwrap())]
    /// Use it to set the topic's replication factor
    pub replication_factor: NonZeroU32,

    #[arg(short, long, required = true)]
    /// The number of partitons of the OUT topic
    pub partitions: String,

    #[arg(long = "for-nbw-strat", action = clap::ArgAction::SetTrue)]
    /// Set it when you intend to use the `NeighborhoodWise` strategy.
    /// It will signal the program to create as many topic as there are
    /// neighborhoods in the geojson neighborhood file specified in the config
    /// (or passed with the -n option). Note that this option must be set even
    /// if the `NeighborhoodWise` strategy is set in the configuration TOML.
    pub for_neighborhoodwise_strategy: bool,

    #[arg(short, long)]
    /// The geojson file to be used to calculate the number of neighborhoods,
    /// and thus partitions, to create.
    pub neighborhood_file: Option<String>,
}

#[derive(Args)]
pub struct TopicDeleteArgs {
    #[arg()]
    /// Kafka topic to delete ("out" only supported at the moment)
    pub topic: String,
}

#[derive(Subcommand)]
pub enum EditConfigCommands {
    #[command(name = "create")]
    /// Creates a default configuration file, or overwrites an existing one,
    /// resetting config to defaults.
    Create(CreateConfig),

    #[command(name = "replace")]
    /// Replace the configuration file with a new one
    Replace(ReplaceConfig),

    #[command(name = "show")]
    /// Shows the current config, if present
    Show,
}

#[derive(Args)]
pub struct ReplaceConfig {
    #[arg()]
    /// File that will replace the current config
    pub file: PathBuf,
}

#[derive(Args)]
pub struct CreateConfig {
    #[arg(long)]
    pub source_topic: Option<String>,

    #[arg(long)]
    pub target_topic: Option<String>,
}
