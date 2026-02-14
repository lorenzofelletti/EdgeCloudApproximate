use std::{
    cell::Cell,
    collections::HashMap,
    error::Error,
    fmt::Debug,
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

use csv::ReaderBuilder;
use geojson::Feature;
use kafka::consumer::Consumer;

use crate::{
    args::CSVProducer,
    config::structs::Config,
    geospatial::{
        get_geohashes_map_from_features, invert_neighborhood_geohashes_map, read_neighborhoods,
    },
    kafka_producer::message::{
        GeoMessage, JSONMessage, JSONMessageDeserialize, WithNeighborhood, LAT_KEY, LON_KEY,
    },
    skip_fail,
    utils::get_topics_names_for_neigborhood_wise_strategy,
};

pub fn run_producer<M>(config: Config, args: &CSVProducer) -> Result<(), Box<dyn Error>>
where
    M: serde::Serialize + Clone + Sync + GeoMessage + JSONMessage + WithNeighborhood,
{
    LAT_KEY.set(args.lat_key.clone()).expect("cannot set lat");
    LON_KEY.set(args.lon_key.clone()).expect("cannot set lon");

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

    let messages: Vec<M> = Vec::<_>::with_capacity(1000);
    let mut messages = Cell::new(messages);

    let base_path = Path::new(&args.out_dir);

    if !base_path.is_dir() {
        return Err(format!(
            "given path \"{}\" is not a directory",
            base_path.to_string_lossy()
        )
        .into());
    }

    println!("Starting to process messages...");

    let mut start_time = std::time::Instant::now();
    loop {
        for message_set in consumer.poll().unwrap().iter() {
            for message in message_set.messages().iter() {
                let mut message = skip_fail!(M::json_deserialize(message.value));
                // set message's geohash and neighborhood
                let gh = skip_fail!(message.geohash());
                message.set_geohash(gh.clone());
                if let Some(n) = geohash_neighborhood_map.get(&gh) {
                    message.set_neighborhood(n.clone());
                }
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
            let mut groups: HashMap<&String, Vec<&M>> = HashMap::new();
            for message in messages.get_mut().iter() {
                let topic = match message.neighborhood().as_ref() {
                    Some(neigh) => neighborhood_files.get(neigh),
                    None => files.last(),
                };

                if let Some(topic) = topic {
                    groups.entry(topic).or_default().push(message);
                }
            }

            for (topic, msgs) in groups {
                let filename = format!("{}.csv", topic);
                let path = PathBuf::new().join(base_path).join(filename);
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

pub fn run_producer_from_file<M>(config: Config, args: &CSVProducer) -> Result<(), Box<dyn Error>>
where
    M: for<'de> serde::Deserialize<'de>
        + serde::Serialize
        + Clone
        + Sync
        + GeoMessage
        + JSONMessageDeserialize
        + Debug
        + WithNeighborhood,
{
    // sets the lat/lon keys
    LAT_KEY.set(args.lat_key.clone()).expect("cannot set lat");
    LON_KEY.set(args.lon_key.clone()).expect("cannot set lon");

    if let Some(name) = args.neighborhood_name_key.as_ref() {
        if !name.is_empty() {
            crate::geospatial::set_neighborhood_name_key(name.clone());
        }
    }

    let sampling_strategy = config.data_out.sampling_strategy;
    let sampling_percentage = args.sampling_percentage;

    let features: Vec<Feature> = read_neighborhoods(&config.data_out.neighborhoods_file)
        .map_err(|e| format!("failed to read neighborhoods: {e}"))?;

    let neighborhood_geohashes_map = get_geohashes_map_from_features(&features);
    let geohash_neighborhood_map = invert_neighborhood_geohashes_map(&neighborhood_geohashes_map);

    let files: Vec<_> =
        get_topics_names_for_neigborhood_wise_strategy(&config.clone(), features.as_slice());
    let neigh_not_found_file = files.last().unwrap();

    // map each neighborhood to a file name
    let neighborhood_files: HashMap<_, String> = neighborhood_geohashes_map
        .keys()
        .zip(files.iter())
        .map(|(n, t)| (n.clone(), t.clone()))
        .collect();

    let file = File::open(PathBuf::from(args.input_file.as_ref().unwrap()))?;

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // read and amend headers
    let headers = reader.headers().expect("cannot read headers");
    let headers = out_headers(headers)?;

    let mut messages: Vec<M> = Vec::with_capacity(args.chunk_size);

    let base_path = Path::new(&args.out_dir);

    if !base_path.is_dir() {
        return Err(format!(
            "given path \"{}\" is not a directory",
            base_path.to_string_lossy()
        )
        .into());
    }

    println!("Starting to process messages...");

    for maybe_record in reader.records() {
        let record = skip_fail!(maybe_record);
        // println!("record: {:?}", record);
        let mut record: M = record.deserialize(None)?;
        // println!("gh: {}", record.geohash().unwrap_or_default());
        // set message's geohash and neighborhood
        let gh = skip_fail!(record.geohash());
        // println!("calculated geohash: {}", gh);
        record.set_geohash(gh.clone());
        if let Some(n) = geohash_neighborhood_map.get(&gh) {
            // println!("calculated neighborhood: {}", n);
            record.set_neighborhood(n.clone());
            // println!(
            //     "retrieved neighborhood: {}",
            //     record.neighborhood().unwrap_or_default()
            // );
        }

        messages.push(record);

        // println!("messages.len(): {}", messages.len());

        let len = messages.len();
        if len == args.chunk_size {
            process(
                sampling_strategy,
                sampling_percentage,
                neigh_not_found_file,
                &neighborhood_files,
                &mut messages,
                base_path,
                len,
                headers.clone(),
            )?;

            messages.clear();
        }
    }
    let len = messages.len();
    process(
        sampling_strategy,
        sampling_percentage,
        neigh_not_found_file,
        &neighborhood_files,
        &mut messages,
        base_path,
        len,
        headers,
    )?;

    Ok(())
}

fn out_headers(
    headers: &csv::StringRecord,
) -> Result<Vec<serde_json::Value>, Box<dyn Error + 'static>> {
    let mut headers: Vec<serde_json::Value> = headers.deserialize(None)?;
    headers.append(&mut vec![
        serde_json::Value::String("geohash".to_owned()),
        serde_json::Value::String("neighborhood".to_owned()),
    ]);
    Ok(headers)
}

fn process<M, S>(
    sampling_strategy: crate::kafka_producer::strategies::SamplingStrategy,
    sampling_percentage: f64,
    neigh_not_found_file: S,
    neighborhood_files: &HashMap<String, String>,
    messages: &mut Vec<M>,
    base_path: &Path,
    len: usize,
    headers: Vec<serde_json::Value>,
) -> Result<(), Box<dyn Error + 'static>>
where
    M: for<'de> serde::Deserialize<'de>
        + serde::Serialize
        + Clone
        + Sync
        + GeoMessage
        + JSONMessageDeserialize
        + Debug
        + WithNeighborhood,
    S: AsRef<str>,
{
    println!("Processing {len} messages");
    let elab_time = std::time::Instant::now();
    sampling_strategy.sample(sampling_percentage, messages);
    println!(
        "Sampling done! (took {}ms)",
        elab_time.elapsed().as_millis()
    );
    let mut groups: HashMap<&String, Vec<&M>> = HashMap::new();

    let neigh_not_found_file = neigh_not_found_file.as_ref().to_owned();

    for message in messages.iter() {
        let topic = match message.neighborhood().as_ref() {
            Some(neigh) => neighborhood_files.get(neigh),
            None => Some(&neigh_not_found_file),
        };

        if let Some(topic) = topic {
            groups.entry(topic).or_default().push(message);
        } else {
            groups
                .entry(&neigh_not_found_file)
                .or_default()
                .push(message);
        }
    }
    for (topic, msgs) in groups {
        let filename = format!("{}.csv", topic);
        let path = PathBuf::new().join(base_path).join(filename);
        let file_exists = path.exists();

        let file = OpenOptions::new().create(true).append(true).open(path)?;

        let mut wtr = csv::WriterBuilder::new().from_writer(file);

        if !file_exists {
            // write the headers
            wtr.serialize(headers.clone())?;
        }

        for msg in msgs {
            wtr.serialize(msg)?;
        }
        wtr.flush()?;
    }
    println!(
        "{} messages stored! (took {}ms)",
        messages.len(),
        elab_time.elapsed().as_millis()
    );
    Ok(())
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
