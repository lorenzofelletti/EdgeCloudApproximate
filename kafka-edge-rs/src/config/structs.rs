use std::time::Duration;

use crate::kafka_producer::strategies::{SamplingStrategy, SendStrategy};

#[derive(Debug, Clone)]
pub struct Kafka {
    pub zookeeper: Vec<String>,
    pub brokers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DataIn {
    pub source_topic: String,
    pub partition_to_read: i32,
}

#[derive(Debug, Clone)]
pub struct DataOut {
    pub target_topic: String,
    pub send_every_ms: Duration,
    pub send_strategy: SendStrategy,
    pub sampling_strategy: SamplingStrategy,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub kafka: Kafka,
    pub data_in: DataIn,
    pub data_out: DataOut,
}
