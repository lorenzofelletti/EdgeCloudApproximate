use std::{
    error::Error,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use kafka::client::KafkaClient;

use crate::{
    args::TopicCreateArgs,
    config::structs::Config,
    geospatial::read_neighborhoods,
    subcommands::{errors::SubcommandError, utils::join_by_comma},
    utils::get_topics_names_for_neigborhood_wise_strategy,
};

fn _topic_create(config: Config, args: &TopicCreateArgs) -> Result<(), Box<dyn Error>> {
    let replication_factor = args.replication_factor;

    let topic: Result<String, Box<dyn Error>> = match args.topic.to_lowercase().as_str() {
        "out" => Ok(config.data_out.target_topic),
        _ => Err("unsupported".into()),
    };
    let topic = topic?;

    // Connect to Kafka and fetch the metadata for the topic
    let mut client = KafkaClient::new(config.kafka.brokers.clone());
    client.load_metadata_all()?;

    if topic_exists(client, &topic) {
        println!("Topic {} exists already!", topic);
        return Ok(());
    }

    let brokers = join_by_comma(&config.kafka.brokers);

    let partitions = &args.partitions;

    // run kafka-topics.sh --create --topic TOPIC --bootstrap-server BROKERS --partiotions PARTITIONS
    // --replication-factor REPLICATION_FACTOR
    let res = Command::new("kafka-topics.sh")
        .arg("--create")
        .arg("--topic")
        .arg(topic)
        .arg("--bootstrap-server")
        .arg(brokers)
        .arg("--partitions")
        .arg(partitions.to_string())
        .arg("--replication-factor")
        .arg(replication_factor.to_string())
        .output()?;

    println!("{}", String::from_utf8_lossy(&res.stdout));

    if ExitStatus::success(&res.status) {
        return Ok(());
    }
    Err(Box::new(SubcommandError::new("Unable to create topic.")))
}

pub fn topic_create(config: &Config, args: &TopicCreateArgs) -> Result<(), Box<dyn Error>> {
    if args.for_neighborhoodwise_strategy == false {
        _topic_create(config.clone(), args)?;
    }

    match args.topic.to_lowercase().as_str() {
        "out" => Ok(()),
        _ => Err("Option --for-nbw-strat unsupported for non out topic."),
    }?;

    // get the right neighborhood file
    let file = match &args.neighborhood_file {
        Some(file) => PathBuf::from(file),
        None => config
            .data_out
            .neighborhoods_file
            .clone()
            .expect("Neigborhood file not supplied and not found in config TOML"),
    };

    let neighborhoods = read_neighborhoods(&file)?;
    let topics_to_create = get_topics_names_for_neigborhood_wise_strategy(config, &neighborhoods);

    let replication_factor = args.replication_factor;
    let partitions = &args.partitions;
    let brokers = join_by_comma(&config.kafka.brokers);
    // leak brokers
    let brokers = brokers.as_str();
    for topic in topics_to_create {
        let res = Command::new("kafka-topics.sh")
            .arg("--create")
            .arg("--topic")
            .arg(topic.clone())
            .arg("--bootstrap-server")
            .arg(brokers)
            .arg("--partitions")
            .arg(partitions.to_string())
            .arg("--replication-factor")
            .arg(replication_factor.to_string())
            .output()?;

        println!("{}", String::from_utf8_lossy(&res.stdout));
        if !ExitStatus::success(&res.status) {
            return Err(format!("Unable to create topic {}", topic).into());
        }
    }

    Ok(())
}

fn topic_exists(client: KafkaClient, topic: &String) -> bool {
    client
        .topics()
        .names()
        .into_iter()
        .any(|t| t.to_owned() == *topic)
}
