pub const TOML_CONFIG_TEMPLATE: &str = "[kafka]\n\
zookeeper = [ \"localhost:2181\" ]\n\
brokers = [ \"localhost:9092\" ]\n\
\n\
[data_in]\n\
source_topic = \"{SOURCE_TOPIC}\"\n\
partition_to_read = \"{PARTITION_TO_READ}\"\n\
\n\
[data_out]\n\
target_topic = \"{SOURCE_TOPIC}\"\n\
send_every_ms = \"{SEND_EVERY_MS}\"\n\
send_strategy = \"{SEND_STRATEGY}\"\n\
sampling_strategy = \"{SAMPLING_STRATEGY}\"\n\
";

pub const TOML_FILE_NAME: &str = "producer_config.toml";

pub const DEFAULT_SOURCE_TOPIC: &str = "datain";
pub const DEFAULT_PARTITION_TO_READ: i32 = 0;
pub const DEFAULT_TARGET_TOPIC: &str = "dataout";
pub const DEFAULT_SEND_EVERY_MS: u64 = 1000;
pub const DEFAULT_SEND_STRATEGY: &str = "RoundRobin";
pub const DEFAULT_SAMPLING_STRATEGY: &str = "Stratified";
