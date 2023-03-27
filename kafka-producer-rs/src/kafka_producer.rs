use std::{error::Error, fs::File, thread::sleep, time::Duration};

use csv::ReaderBuilder;
use kafka::producer::{Producer, Record, RequiredAcks};

use crate::{args::CliArgs, config::structs::Config};

use self::message::Message;

pub mod message;

pub fn run_kafka_producer(config: Config, _cli: &CliArgs) -> Result<(), Box<dyn Error>> {
    let mut producer = Producer::from_hosts(config.kafka.brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    let partitions_number = config.kafka.partitions.get() as i32;

    let file = File::open(config.data.source)?;

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let chunk_size: usize = config.data.chunk_size.get() as usize;
    let mut chunk = Vec::with_capacity(chunk_size);

    for result in reader.records() {
        let record = result?;

        chunk.push(record);

        if chunk.len() == chunk_size {
            let mut i = 0;

            for record in &chunk[..] {
                let data: Message = record.deserialize(None)?;
                let data_json = data.json_serialize();
                let partition: i32 = i % partitions_number; // round robin partition selection

                let record =
                    Record::from_key_value(&config.kafka.topic[..], data.id, data_json.to_string())
                        .with_partition(partition);

                producer.send(&record)?; // send to kafka

                sleep(config.data.msg_sleep_in_ms);

                i += 1;
            }

            chunk.clear();
            sleep(config.data.chunk_sleep_in_ms);
        }
    }

    Ok(())
}
