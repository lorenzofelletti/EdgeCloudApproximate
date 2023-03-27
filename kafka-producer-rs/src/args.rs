use std::{num::NonZeroU32, path::PathBuf};

use clap::{Args, Parser, Subcommand};

const ABOUT: &str = "Kafka CSV Producer \n
Reads from a csv file data to send to a Kafka topic. \n
The csv must contain data with the following structure: \n
    - id    - String \n
    - lat   - Double \n
    - lon   - Double \n
    - time  - String \n
    - speed - Double \n
It uses a TOML configuration file, placed in the same directory as the executable, \
for configuration.";

#[derive(Parser)]
#[command(author, version, about = ABOUT, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub subcommands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "create_topic")]
    /// Create the topic specified in the configuration file (if not already created)
    CreateTopic(CreateTopic),

    #[command(name = "delete_topic")]
    /// Delete the topic specified in the configuration file (if it exists)
    DeleteTopic,

    #[command(name = "edit_config")]
    /// Edit configuration
    EditConfig(EditConfig),
}

#[derive(Args)]
pub struct CreateTopic {
    #[arg(long, default_value_t = NonZeroU32::new(1).unwrap())]
    /// Use it to set the topic's replication factor
    pub replication_factor: NonZeroU32,
}

#[derive(Args)]
pub struct EditConfig {
    #[command(subcommand)]
    pub subcommands: EditConfigCommands,
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
