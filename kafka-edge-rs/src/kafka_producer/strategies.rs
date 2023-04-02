#[derive(Debug, Clone, Copy)]
/// Enum of the possible strategies to send messages to Kafka partitions.
pub enum SendStrategy {
    /// Send messages to a random partition.
    Random,
    /// Send messages to partitions in a round-robin fashion.
    RoundRobin,
}

pub enum SamplingStrategy {
    /// Sample messages randomly.
    Random,
    /// Sample messages using stratified sampling.
    Stratified,
}
