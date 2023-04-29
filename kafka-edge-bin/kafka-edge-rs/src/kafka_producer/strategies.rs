use std::collections::HashMap;

use kafka::producer::{Producer, Record};

use rand::{seq::IteratorRandom, Rng};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{create_record, create_record_with_partition, either};

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
        neighborhood_topics: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            SendStrategy::NeighborhoodWise => {
                let records = messages
                    .par_iter()
                    .filter_map(|message| {
                        let topic = match message.neighborhood.as_ref() {
                            Some(neigh) => neighborhood_topics.get(neigh),
                            None => topics.last(),
                        }
                        .unwrap();
                        Some(create_record!(topic, message))
                    })
                    .collect::<Vec<_>>();

                for chunk in records.chunks(100_000) {
                    producer.send_all(chunk)?;
                }
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
                        partition = rand::thread_rng().gen_range(0..partitions);
                    }

                    // create a record
                    let record = create_record_with_partition!(topic, msg, partition);

                    // send the record
                    producer.send(&record)?;
                }
            }
        }
        Ok(())
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

    /// Samples the given messages using the given sampling percentage.
    /// A sampling percentage of 0 means that all messages are kept.
    /// A sampling percentage of 1 means that all messages are discarded.
    ///
    /// # Arguments
    /// * `sampling_percentage` - The percentage of messages to keep.
    /// * `messages` - The messages to sample.
    ///
    /// # Notes
    /// The `messages` vector is modified in-place.
    pub fn sample(&self, sampling_percentage: f64, messages: &mut Vec<Message>) {
        if sampling_percentage == 1.0 || messages.is_empty() {
            return;
        }
        if sampling_percentage == 0.0 {
            messages.clear();
            return;
        }

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
                let total_size = messages.len();
                let sample_size = total_size as f64 * sampling_percentage;
                let mut groups: HashMap<&str, Vec<&Message>> = HashMap::new();
                for message in messages as &[Message] {
                    groups
                        .entry(message.geohash.as_ref().unwrap())
                        .or_insert_with(Vec::new)
                        .push(message);
                }

                let mut sample_sizes: HashMap<&str, usize> = HashMap::new();
                for (geohash, group) in &groups {
                    if group.len() == 1 {
                        sample_sizes.insert(
                            geohash,
                            either!(rng.gen_bool(sampling_percentage) => 1; 0) as usize,
                        );
                    } else {
                        let proportion = group.len() as f64 / total_size as f64;
                        sample_sizes.insert(geohash, (proportion * sample_size) as usize);
                    }
                }

                let sampled_groups: Vec<Vec<&Message>> = groups
                    .par_iter()
                    .map(|(geohash, group)| {
                        group
                            .iter()
                            .cloned()
                            .choose_multiple(&mut rand::thread_rng(), sample_sizes[geohash])
                    })
                    .collect();
                *messages = sampled_groups.into_iter().flatten().cloned().collect();
            }
        }
    }
}

/// tests the sampling strategy
#[cfg(test)]
mod tests {
    use geo::Coord;

    use super::*;

    fn get_random_vec_msgs_of_len(len: usize) -> Vec<Message> {
        let mut msgs = Vec::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            let mut msg = Message {
                id: "".to_string(),
                lat: 114.0 + rng.gen_range(-0.05..0.05),
                lon: 22.5 + rng.gen_range(-0.05..0.05),
                time: "".to_string(),
                speed: 46.0,
                geohash: None,
                neighborhood: None,
            };
            msg.geohash = Some(msg.geohash().unwrap());
            msgs.push(msg);
        }
        msgs
    }

    fn sample_msgs(
        msgs_len: usize,
        strat: SamplingStrategy,
        sampling_percentage: f64,
    ) -> Vec<Message> {
        let mut msgs = get_random_vec_msgs_of_len(msgs_len);

        dbg!(msgs.len());
        strat.sample(sampling_percentage, &mut msgs);
        dbg!(msgs.len());
        msgs
    }

    #[test]
    fn test_stratified_sampling_sample_0_50() -> Result<(), Box<dyn std::error::Error>> {
        let strat = SamplingStrategy::Stratified;

        let msgs_len = 10000;
        let sampling_percentage = 0.50;
        let to_retain = (msgs_len / 2) as f64;

        let msgs = sample_msgs(msgs_len, strat, sampling_percentage);

        assert!(msgs.len() < msgs_len);
        assert!((msgs.len() as f64) > to_retain - msgs_len as f64 * 0.05);
        assert!((msgs.len() as f64) < to_retain + msgs_len as f64 * 0.05);

        Ok(())
    }

    #[test]
    fn test_stratified_sampling_sample_0_90() -> Result<(), Box<dyn std::error::Error>> {
        let strat = SamplingStrategy::Stratified;

        let msgs_len = 10000;
        let sampling_percentage = 0.90;
        let to_retain = msgs_len as f64 * sampling_percentage;

        let msgs = sample_msgs(msgs_len, strat, sampling_percentage);

        assert!(msgs.len() < msgs_len);
        assert!((msgs.len() as f64) > to_retain - msgs_len as f64 * 0.05);
        assert!((msgs.len() as f64) < to_retain + msgs_len as f64 * 0.05);

        Ok(())
    }

    #[test]
    fn test_stratified_sampling_sample_0_10() -> Result<(), Box<dyn std::error::Error>> {
        let strat = SamplingStrategy::Stratified;

        let msgs_len = 10000;
        let sampling_percentage = 0.10;
        let to_retain = msgs_len as f64 * sampling_percentage;

        let msgs = sample_msgs(msgs_len, strat, sampling_percentage);

        assert!(msgs.len() < msgs_len);
        assert!((msgs.len() as f64) > to_retain - msgs_len as f64 * 0.05);
        assert!((msgs.len() as f64) < to_retain + msgs_len as f64 * 0.05);

        Ok(())
    }

    #[test]
    fn test_stratified_sampling_sample_1_0() -> Result<(), Box<dyn std::error::Error>> {
        let strat = SamplingStrategy::Stratified;

        let msgs_len = 10000;
        let sampling_percentage = 1.0;

        let msgs = sample_msgs(msgs_len, strat, sampling_percentage);

        assert!(msgs.len() == msgs_len);
        Ok(())
    }

    #[test]
    fn test_stratified_sampling_equal_array() -> Result<(), Box<dyn std::error::Error>> {
        let strat = SamplingStrategy::Stratified;

        let msgs_len = 10000;
        let sampling_percentage = 0.40;
        let to_retain = msgs_len as f64 * sampling_percentage;

        let mut msgs = vec![
            Message {
                id: "".to_string(),
                lat: 114.0,
                lon: 22.5,
                time: "".to_string(),
                speed: 46.0,
                geohash: Some(geohash::encode(Coord { x: 114.0, y: 22.5 }, 6).unwrap()),
                neighborhood: None,
            };
            msgs_len
        ];

        dbg!(msgs.len());
        strat.sample(sampling_percentage, &mut msgs);
        dbg!(msgs.len());

        assert!(msgs.len() < msgs_len);
        assert!((msgs.len() as f64) > to_retain - msgs_len as f64 * 0.05);
        assert!((msgs.len() as f64) < to_retain + msgs_len as f64 * 0.05);

        Ok(())
    }

    #[test]
    fn test_stratified_sampling_many_groups_of_one() {
        let strat = SamplingStrategy::Stratified;

        let msgs_len = 10000;
        let sampling_percentage = 0.40;
        let to_retain = msgs_len as f64 * sampling_percentage;

        let mut rng = rand::thread_rng();
        let mut msgs = Vec::with_capacity(msgs_len);
        for _ in 0..msgs_len {
            let mut msg = Message {
                id: "".to_string(),
                lat: 114.0 + rng.gen_range(-10.0..=10.0),
                lon: 22.5 + rng.gen_range(-5.0..=5.0),
                time: "".to_string(),
                speed: 46.0,
                geohash: None,
                neighborhood: None,
            };
            msg.geohash = Some(msg.geohash().unwrap());
            msgs.push(msg);
        }

        dbg!(msgs.len());
        strat.sample(sampling_percentage, &mut msgs);
        dbg!(msgs.len());

        assert!(msgs.len() < msgs_len);
        assert!((msgs.len() as f64) > to_retain - msgs_len as f64 * 0.05);
        assert!((msgs.len() as f64) < to_retain + msgs_len as f64 * 0.05);
    }

    #[test]
    fn test_random_sampling_sample_0_90() -> Result<(), Box<dyn std::error::Error>> {
        let strat = SamplingStrategy::Random;

        let msgs_len = 10000;
        let sampling_percentage = 0.90;
        let to_retain = msgs_len as f64 * sampling_percentage;

        let msgs = sample_msgs(msgs_len, strat, sampling_percentage);

        assert!(msgs.len() < msgs_len);
        assert!((msgs.len() as f64) > to_retain - msgs_len as f64 * 0.05);
        assert!((msgs.len() as f64) < to_retain + msgs_len as f64 * 0.05);

        Ok(())
    }
}
