use std::collections::HashMap;

use geojson::Feature;
use kafka::producer::{Producer, Record};
use rand::Rng;

use super::message::Message;

#[derive(Debug, Clone, Copy)]
/// Enum of the possible strategies to send messages to Kafka partitions.
pub enum SendStrategy {
    /// Send messages to a random partition.
    Random,
    /// Send messages to partitions in a round-robin fashion.
    RoundRobin,
    /// Send messages to partition, where each partition is assigned a
    /// neighborhood, and only receives data from that neighborhood.
    ///
    /// # Notes
    /// ## How does this strategy work?
    /// Note that this strategy will output data to `N` different topis, where
    /// `N` is the number of neighborhoods read from the file, named in
    /// ascending number (from 1). The topics names share the `taget_topic`
    /// string set in the configuration TOML file as common prefix.
    ///
    /// For example, if the config file contains `taget_topic = "dataout-"`,
    /// the output topics will be named `dataout-1`, `dataout-1`, etc.
    ///
    /// ## Are topics automatically created?
    /// No, the topics are not automatically created. You need to either create
    /// them manualy beforhand, or run this program with
    /// `topic create out --for-nbw-strat` a first time before running it to
    /// send data.
    NeighborhoodWise,
}

#[derive(Debug, Clone, Copy)]
pub enum SamplingStrategy {
    /// Sample messages randomly.
    Random,
    /// Sample messages using stratified sampling.
    Stratified,
}

impl SendStrategy {
    /// Tries to parse the given strategy string to a `SendStrategy`.
    pub fn parse_send_strategy<S: Into<String>>(strategy: S) -> Option<SendStrategy> {
        let strategy: String = strategy.into();
        match strategy.to_lowercase().as_str() {
            "random" => Some(SendStrategy::Random),
            "roundrobin" => Some(SendStrategy::RoundRobin),
            "neighborhoodwise" => Some(SendStrategy::NeighborhoodWise),
            _ => None,
        }
    }

    /// Send the given messages to the given producer.
    /// The messages are sent to the partition determined by the strategy.
    /// The messages are JSON serialized before being sent.
    ///
    /// # Arguments
    /// * `producer` - The producer to send the messages to.
    /// * `messages` - The messages to send.
    /// * `topics` - The topics among which to decide to send.
    /// * `partitions` - The partitions to which to send to (it will be ignored
    ///   by [SendStrategy::NeighborhoodWise]).
    ///
    /// # Notes
    /// Only the first topic in the `topics` slice will be considered for all
    /// strategies but [SendStrategy::NeighborhoodWise].
    pub fn send(
        self,
        producer: &mut Producer,
        messages: &Vec<Message>,
        topics: &[String],
        partitions: i32,
        features: &Option<Vec<Feature>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SendStrategy::NeighborhoodWise => {
                let topic = topics.first().expect("No topic given!");
                let mut neighborhoods: HashMap<String, Vec<Message>> = HashMap::new();
                let x = features.unwrap()[0];
                let x =x.geometry.unwrap();
                let x = x.bbox.unwrap();

                // group messages by neighborhood
                for msg in messages {
                    let neighborhood = msg
                        .get_neighborhood(&features.unwrap())
                        .expect("No neighborhood found for message!");
                    let neighborhood = neighborhood.to_string();
                    let neighborhood = neighborhoods.entry(neighborhood).or_insert(vec![]);
                    neighborhood.push(msg.clone());
                }

                // send messages to their respective neighborhood
                for (idx, (neighborhood, messages)) in neighborhoods.iter().enumerate() {
                    let topic = format!("{}{}", topic, idx + 1);
                    let mut partition: i32;

                    for (idx, msg) in messages.iter().enumerate() {
                        // choose a partition in a round-robin fashion
                        partition = (idx % partitions as usize) as i32;

                        // create a record
                        let record = Record::from_key_value(
                            &topic,
                            msg.id.clone(),
                            msg.json_serialize().to_string(),
                        )
                        .with_partition(partition);

                        // send the record
                        producer.send(&record)?;
                    }
                }
                Ok(())
            }
            strat => {
                let topic = topics.first().expect("No topic given!");
                let mut partition: i32;

                for (idx, msg) in messages.iter().enumerate() {
                    if matches!(strat, SendStrategy::RoundRobin) {
                        // choose a partition in a round-robin fashion
                        partition = (idx % partitions as usize) as i32;
                    } else {
                        // choose a random partition
                        partition = rand::thread_rng()
                            .gen_range(0..partitions)
                            .try_into()
                            .unwrap();
                    }

                    // create a record
                    let record = Record::from_key_value(
                        &topic,
                        msg.id.clone(),
                        msg.json_serialize().to_string(),
                    )
                    .with_partition(partition);

                    // send the record
                    producer.send(&record)?;
                }
                Ok(())
            }
        }
    }
}

impl SamplingStrategy {
    /// Tries to parse the given strategy string to a `SamplingStrategy`.
    pub fn _parse_sampling_strategy<S: Into<String>>(strategy: S) -> Option<SamplingStrategy> {
        let strategy: String = strategy.into();
        match strategy.to_lowercase().as_str() {
            "random" => Some(SamplingStrategy::Random),
            "stratified" => Some(SamplingStrategy::Stratified),
            _ => None,
        }
    }

    pub fn sample(&self, sampling_percentage: f64, messages: &mut Vec<Message>) {
        let mut rng = rand::thread_rng();
        match self {
            SamplingStrategy::Random => {
                messages.retain(|_| rng.gen_bool(sampling_percentage));
            }
            SamplingStrategy::Stratified => {
                /* applies stratified sampling to the given messages,
                 * using the geohash of the message as the stratification
                 * calculated by the lat and lon of the message.
                 * The messages are grouped by geohash, and then a random
                 * sample is taken from each group. */

                // create a map of geohash to indexes of messages with that geohash
                let mut geohash_msgs_idx_map: HashMap<String, Vec<usize>> = HashMap::new();
                for (i, msg) in messages.iter().enumerate() {
                    let geohash = msg.geohash().unwrap_or_default().clone();
                    let idx = geohash_msgs_idx_map.entry(geohash).or_insert(vec![]);
                    idx.push(i);
                }

                // sample each group
                geohash_msgs_idx_map.iter_mut().for_each(|(_, idxs)| {
                    idxs.retain(|_| rng.gen_bool(sampling_percentage));
                });
                // flatten the map to a vector of indexes of messages to keep
                let mut to_keep: Vec<usize> = geohash_msgs_idx_map
                    .values()
                    .flatten()
                    .map(|i| *i)
                    .collect();
                to_keep.sort();

                // retain only the messages with the indexes in `to_keep`
                let mut i = 0;
                messages.retain(|_| {
                    i += 1;
                    to_keep.contains(&(i - 1))
                });
            }
        }
    }
}
