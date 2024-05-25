use std::{cell::Cell, collections::HashMap, error::Error, time::Duration, vec};

use geojson::Feature;
use kafka::{
    consumer::Consumer,
    producer::{Producer, RequiredAcks},
};
use log::warn;

use crate::{
    args::CliArgs,
    config::structs::Config,
    geospatial::{
        get_geohashes_map_from_features, invert_neighborhood_geohashes_map, read_neighborhoods,
    },
    skip_fail,
    utils::get_topics_names_for_neigborhood_wise_strategy,
};

use self::{message::Message, strategies::SendStrategy};

pub mod message;
pub mod strategies;
pub mod utils;

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
        .with_offset_storage(Some(kafka::consumer::GroupOffsetStorage::Kafka))
        .with_group(config.data_in.consumer_group)
        .with_topic_partitions(
            config.data_in.source_topic,
            &config.data_in.partitions_to_consume,
        )
        .create()
}

fn get_topics_names(config: &Config, features: &[Feature], strategy: SendStrategy) -> Vec<String> {
    match strategy {
        SendStrategy::NeighborhoodWise => {
            get_topics_names_for_neigborhood_wise_strategy(config, features)
        }
        _ => {
            // target_topic feature.len() times
            vec![config.data_out.target_topic.clone(); features.len()]
        }
    }
}

fn send_strategy_to_use(args: &CliArgs, config: &Config) -> Option<SendStrategy> {
    match &args.override_send_strategy {
        Some(strategy) => SendStrategy::parse_send_strategy(strategy),
        None => Some(config.data_out.send_strategy),
    }
}

fn load_producer_metadata(producer: &mut Producer, topics: &[String]) -> Result<(), kafka::Error> {
    producer.client_mut().load_metadata(topics)
}

fn load_consumer_metadata(config: &Config, consumer: &mut Consumer) -> Result<(), kafka::Error> {
    consumer
        .client_mut()
        .load_metadata(&[config.data_in.source_topic.clone()])
}

pub fn run_producer(config: Config, args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let sampling_strategy = config.data_out.sampling_strategy;
    let sampling_percentage = args.sampling_percentage;

    let send_strategy = send_strategy_to_use(args, &config).ok_or("Unrecognized strategy")?;

    let features: Vec<Feature> = read_neighborhoods(&config.data_out.neighborhoods_file)?;

    let topics = get_topics_names(&config, &features, send_strategy);

    let neighborhood_geohashes_map = get_geohashes_map_from_features(&features);
    let geohash_neighborhood_map = invert_neighborhood_geohashes_map(&neighborhood_geohashes_map);
    // map each neighborhood to a topic name (topic names and neighborhood names iterate in parallel)
    let neighborhood_topics: HashMap<String, String> = neighborhood_geohashes_map
        .keys()
        .zip(topics.iter())
        .map(|(n, t)| (n.clone(), t.clone()))
        .collect();

    let mut consumer = make_consumer(config.clone())?;
    let mut producer = make_producer(config.clone())?;

    // loads the metadata needed for the client and producer
    load_consumer_metadata(&config, &mut consumer)?;
    load_producer_metadata(&mut producer, &topics)?;

    // partitions will be ignored in neighborhoodwise strategy
    let fetched_topics = producer.client().topics();
    let partitions = fetched_topics.partitions(&topics[0]);
    let partitions = match partitions {
        Some(p) => p.len() as i32,
        None => 0,
    };

    let messages: Vec<Message> = Vec::<_>::with_capacity(1000);
    let mut messages = Cell::new(messages);

    println!("Starting to process messages...");

    let mut start_time = std::time::Instant::now();
    loop {
        for message_set in consumer.poll().unwrap().iter() {
            for message in message_set.messages().iter() {
                let mut message = skip_fail!(Message::json_deserialize(message.value));
                // set message's geohash and neighborhood
                let gh = skip_fail!(message.geohash());
                message.geohash = Some(gh.clone());
                message.neighborhood = geohash_neighborhood_map.get(&gh).cloned();
                messages.get_mut().push(message);
            }
        }
        if start_time.elapsed().as_millis() >= config.data_out.send_every_ms.as_millis() {
            println!("Processing {} messages", messages.get_mut().len());
            let elab_time = std::time::Instant::now();
            sampling_strategy.sample(sampling_percentage, messages.get_mut());
            println!(
                "Sampling done! (took {}ms)",
                elab_time.elapsed().as_millis()
            );
            send_strategy.send(
                &mut producer,
                messages.get_mut(),
                &topics,
                partitions,
                &neighborhood_topics,
            )?;
            println!(
                "{} messages sent! (took {}ms)",
                messages.get_mut().len(),
                elab_time.elapsed().as_millis()
            );
            messages.get_mut().clear();
            start_time = std::time::Instant::now();
        }
    }
}
