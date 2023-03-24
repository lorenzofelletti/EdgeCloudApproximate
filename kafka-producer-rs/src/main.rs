use std::{path::Path, thread::sleep, time::Duration};

use kafka::{
    producer::{Producer, RequiredAcks},
};
use kafka_producer::{errors::KafkaSendError, message::Message};

mod config;
mod kafka_producer;

const BROKERS: [&str; 1] = ["192.168.56.46:9092"];
const BASE_PATH: &str = "/home/lorenzo/EdgeCloudApproximate/";
const FILE_RELATIVE_PATH: &str = "data/china/mobility/guang.csv";
const CHUNK_SLEEP_IN_S: Duration = Duration::from_secs(5 * 60);
const KAFKA_MSG_SLEEP_IN_MS: Duration = Duration::from_millis(1);

fn send_to_kafka_topic(
    msg: Message,
    topic: String,
    partition: Option<String>,
    brokers: Vec<String>,
) -> Result<(), KafkaSendError> {
    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create();
    Err(KafkaSendError::new_unknown_error("Error sending the message"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_path = BASE_PATH.to_owned();
    file_path.push_str(FILE_RELATIVE_PATH);
    let mut reader = csv::Reader::from_path(Path::new(&file_path)).unwrap();

    for row in reader.records() {
        match row {
            Ok(row) => {
                let data: Message = row.deserialize(None)?;
                //let brokers = brokers.into_iter().map(|x| x.to_owned()).collect();
                //send_to_kafka_topic(msg, topic, partition, brokers);
                if !KAFKA_MSG_SLEEP_IN_MS.is_zero() {
                    sleep(KAFKA_MSG_SLEEP_IN_MS)
                };
            }
            Err(e) => {
                println!("Error reading row: `{e}'");
                continue;
            }
        }
    }

    Ok(())
}
