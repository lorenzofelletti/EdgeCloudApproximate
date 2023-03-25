use std::{num::NonZeroU64, path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct Kafka {
    pub zookeeper: Vec<String>,
    pub brokers: Vec<String>,
    pub topic: String,
    pub partitions: NonZeroU64,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub source: PathBuf,
    pub msg_sleep_in_ms: Duration,
    pub chunk_size: NonZeroU64,
    pub chunk_sleep_in_ms: Duration,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub kafka: Kafka,
    pub data: Data,
}
