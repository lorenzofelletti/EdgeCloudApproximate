use std::{cell::Cell, collections::HashMap, error::Error, fs::OpenOptions, path::Path};

use geojson::Feature;
use kafka::consumer::Consumer;

use crate::{
    args::CSVProducer,
    config::structs::Config,
    geospatial::{
        get_geohashes_map_from_features, invert_neighborhood_geohashes_map, read_neighborhoods,
    },
    kafka_producer::message::{GeoMessage as _, JSONMessage as _, Message},
    skip_fail,
    utils::get_topics_names_for_neigborhood_wise_strategy,
};

pub fn run_producer(config: Config, args: &CSVProducer) -> Result<(), Box<dyn Error>> {
    let sampling_strategy = config.data_out.sampling_strategy;
    let sampling_percentage = args.sampling_percentage;

    let features: Vec<Feature> = read_neighborhoods(&config.data_out.neighborhoods_file)?;

    let neighborhood_geohashes_map = get_geohashes_map_from_features(&features);
    let geohash_neighborhood_map = invert_neighborhood_geohashes_map(&neighborhood_geohashes_map);

    let files: Vec<_> =
        get_topics_names_for_neigborhood_wise_strategy(&config.clone(), features.as_slice());

    // map each neighborhood to a file name
    let neighborhood_files: HashMap<_, String> = neighborhood_geohashes_map
        .keys()
        .zip(files.iter())
        .map(|(n, t)| (n.clone(), t.clone()))
        .collect();

    let mut consumer = make_consumer(config.clone())?;
    load_consumer_metadata(&config, &mut consumer)?;

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

            // save to csv according to neighborhood
            let mut groups: HashMap<&String, Vec<&Message>> = HashMap::new();
            for message in messages.get_mut().iter() {
                let topic = match message.neighborhood.as_ref() {
                    Some(neigh) => neighborhood_files.get(neigh),
                    None => files.last(),
                };

                if let Some(topic) = topic {
                    groups.entry(topic).or_default().push(message);
                }
            }

            for (topic, msgs) in groups {
                let filename = format!("{}.csv", topic);
                let path = Path::new(&filename);
                let file_exists = path.exists();

                let file = OpenOptions::new().create(true).append(true).open(path)?;

                let mut wtr = csv::WriterBuilder::new()
                    .has_headers(!file_exists)
                    .from_writer(file);

                for msg in msgs {
                    wtr.serialize(msg)?;
                }
                wtr.flush()?;
            }

            println!(
                "{} messages stored! (took {}ms)",
                messages.get_mut().len(),
                elab_time.elapsed().as_millis()
            );
            messages.get_mut().clear();
            start_time = std::time::Instant::now();
        }
    }
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

fn load_consumer_metadata(config: &Config, consumer: &mut Consumer) -> Result<(), kafka::Error> {
    consumer
        .client_mut()
        .load_metadata(&[config.data_in.source_topic.clone()])
}
