pub const TOML_CONFIG_TEMPLATE: &str = "[kafka]\n\
zookeeper = [ \"localhost:2181\" ]\n\
brokers = [ \"localhost:9092\" ]\n\
topic = \"datain\"\n\
partitions = 4\n\
\n\
[data]\n\
source = \"{data_source}\"\n\
msg_sleep_in_ms = 1\n\
chunk_size = 300000\n\
chunk_sleep_in_ms = 300000 # 5 minutes\n\
";

pub const TOML_FILE_NAME: &str = "producer_config.toml";
