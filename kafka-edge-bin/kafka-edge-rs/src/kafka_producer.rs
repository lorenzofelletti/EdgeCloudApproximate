use std::{error::Error, time::Duration};

use kafka::{
    consumer::Consumer,
    producer::{Producer, RequiredAcks},
};

use crate::{args::CliArgs, config::structs::Config};

mod message;
pub mod strategies;

mod macros {
    #[macro_export]
    macro_rules! make_producer {
        () => {};
    }
}

fn make_producer(config: Config) -> Result<Producer, kafka::Error> {
    Producer::from_hosts(config.kafka.brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
}

fn make_consumer(config: Config) -> Result<Consumer, kafka::Error> {
    Consumer::from_hosts(config.kafka.brokers)
        .with_topic_partitions(
            config.data_in.source_topic,
            &[config.data_in.partition_to_read],
        )
        .create()
}

pub fn run_producer(config: Config, args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let mut consumer = make_consumer(config.clone())?;
    let mut producer = make_producer(config.clone())?;

    // reads messages from consumer for 5 minutes
    // then sleeps and starts again after 5 minutes
    Ok(())
}
