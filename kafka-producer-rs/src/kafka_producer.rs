use std::{
    error::Error,
    fs::File,
    thread::sleep,
    time::{Duration, Instant},
};

use csv::ReaderBuilder;
use kafka::producer::{Producer, Record, RequiredAcks};

use crate::{args::CliArgs, config::structs::Config};

use self::message::Message;

pub mod message;

/// Produces Kafka messages on the topic indicated in the configuration at a specified rate,
/// also indicated in the configuration.
/// The function does not take care of the topic's creation, thus assumes it is already.
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
    let mut chunk_records = Vec::with_capacity(chunk_size);

    let mut start_time = Instant::now();
    let mut partition = 0;
    for result in reader.records() {
        let record = result?;

        chunk.push(record);

        if chunk.len() == chunk_size {
            for record in &chunk[..] {
                let data: Message = record.deserialize(None)?;
                let data_json = data.json_serialize();
                chunk_records.push(
                    Record::from_key_value(&config.kafka.topic[..], data.id, data_json.to_string())
                        .with_partition(partition),
                );
            }

            // wait for all the chunk_sleep_in_ms to pass
            let elapsed = start_time.elapsed();
            if start_time.elapsed() < config.data.chunk_sleep_in_ms {
                sleep(config.data.chunk_sleep_in_ms - elapsed);
            }

            // send the chunk
            for rec_chunk in chunk_records.chunks(100) {
                producer.send_all(&rec_chunk)?;
            }

            println!("Sent {} records", chunk_records.len());

            // reset for next chunk
            chunk.clear();
            partition = (partition + 1) % partitions_number;
            start_time = Instant::now();
        }
    }

    Ok(())
}
