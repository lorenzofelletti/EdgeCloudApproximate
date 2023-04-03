use std::{path::PathBuf, num::NonZeroU32};

use clap::{Args, Parser, Subcommand};

const ABOUT: &str = "Kafka Edge Producer \n
Reads from a Kafka IN topic data, samples it, and then sends it to a Kafka OUT topic. \n
It uses a TOML configuration file, placed in the same directory as the executable, \
for configuration.";

#[derive(Parser)]
#[command(author, version, about = ABOUT, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[arg(long)]
    pub override_send_strategy: Option<String>,

    #[command(subcommand)]
    pub subcommands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "topic")]
    /// Manage out topic
    Topic(Topic),

    #[command(name = "config")]
    /// Create or edit configuration
    EditConfig(EditConfig),
}

#[derive(Args)]
pub struct Topic {
    #[command(subcommand)]
    pub subcommands: Option<TopicCommands>,
}

#[derive(Subcommand)]
pub enum TopicCommands {
    Create(TopicCreateArgs),
}

#[derive(Args)]
pub struct EditConfig {
    #[command(subcommand)]
    pub subcommands: EditConfigCommands,
}

#[derive(Args)]
pub struct TopicCreateArgs {
    #[arg(long, default_value_t = NonZeroU32::new(1).unwrap())]
    /// Use it to set the topic's replication factor
    pub replication_factor: NonZeroU32,

    #[arg(short, long, required = true)]
    /// The number of partitons of the OUT topic
    pub partitions: String,
}

#[derive(Subcommand)]
pub enum EditConfigCommands {
    #[command(name = "create")]
    /// Creates a default configuration file, or overwrites an existing one,
    /// resetting config to defaults.
    Create,

    #[command(name = "replace")]
    /// Replace the configuration file with a new one
    Replace(ReplaceConfig),
}

#[derive(Args)]
pub struct ReplaceConfig {
    #[arg()]
    /// File that will replace the current config
    pub file: PathBuf,
}
