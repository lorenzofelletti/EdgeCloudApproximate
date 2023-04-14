use std::{error::Error, time::Duration, vec};

use geojson::Feature;
use kafka::{
    consumer::{Consumer, MessageSet},
    producer::{Producer, RequiredAcks},
};

use crate::{
    args::CliArgs, config::structs::Config, geospatial::read_neighborhoods,
    utils::get_topics_names_for_neigborhood_wise_strategy,
};

use self::{message::Message, strategies::SendStrategy};

pub mod message;
pub mod strategies;

mod macros {
    #[macro_export]
    macro_rules! make_producer {
        () => {};
    }
}

/// Create a Kafka producer from the given config.
fn make_producer(config: Config) -> Result<Producer, kafka::Error> {
    Producer::from_hosts(config.kafka.brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
}

/// Create a Kafka consumer from the given config.
fn make_consumer(config: Config) -> Result<Consumer, kafka::Error> {
    Consumer::from_hosts(config.kafka.brokers)
        .with_fallback_offset(kafka::consumer::FetchOffset::Earliest)
        .with_offset_storage(kafka::consumer::GroupOffsetStorage::Kafka)
        .with_group(config.data_in.consumer_group)
        .with_topic(config.data_in.source_topic)
        .create()
}

fn map_message_set_to_messages(message_set: &MessageSet) -> Vec<Message> {
    let messages: Vec<Result<Message, serde_json::Error>> = message_set
        .messages()
        .iter()
        .map(|m| Message::json_deserialize(m.value.into()))
        .collect();
    messages
        .iter()
        .filter(|m| m.is_ok())
        .map(|m| m.as_ref().unwrap().clone())
        .collect()
}

pub fn run_producer(config: Config, args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let sampling_strategy = config.data_out.sampling_strategy;
    let sampling_percentage = args.sampling_percentage;

    let send_strategy = match &args.override_send_strategy {
        Some(strategy) => SendStrategy::parse_send_strategy(strategy),
        None => Some(config.data_out.send_strategy),
    }
    .ok_or("Unrecognized strategy")?;

    let mut features: Option<Vec<Feature>> = None;

    let output_topics = match send_strategy {
        SendStrategy::NeighborhoodWise => {
            features =
                Some(read_neighborhoods(
                    &config.clone().data_out.neighborhoods_file.expect(
                        "Neighborhoods file must exists for this NeighborhooWise strategy!",
                    ),
                )?);
            get_topics_names_for_neigborhood_wise_strategy(&config, &features.clone().unwrap())
        }
        _ => {
            vec![config.data_out.target_topic.clone()]
        }
    };

    let mut consumer = make_consumer(config.clone())?;
    let mut producer = make_producer(config.clone())?;

    // loads the metadata needed for the client and producer
    consumer
        .client_mut()
        .load_metadata(&[config.data_in.source_topic])?;

    producer
        .client_mut()
        .load_metadata(&output_topics.clone())?;

    // partitions will be ignored in neighborhoodwise strategy
    let topics = producer.client().topics();
    let partitions = topics.partitions(&output_topics[0]);
    let partitions = match partitions {
        Some(p) => p.len() as i32,
        None => 0,
    };

    let mut messages: Vec<Message> = vec![];

    let mut start_time = std::time::Instant::now();
    loop {
        for message_set in consumer.poll().unwrap().iter() {
            for message in map_message_set_to_messages(&message_set) {
                messages.push(message);
            }
        }
        if start_time.elapsed().as_millis() >= config.data_out.send_every_ms.as_millis() {
            sampling_strategy.sample(sampling_percentage, &mut messages);
            send_strategy.send(
                &mut producer,
                &messages,
                &output_topics,
                partitions,
                &features,
            )?;
            messages.clear();
            start_time = std::time::Instant::now();
        }
    }
}
