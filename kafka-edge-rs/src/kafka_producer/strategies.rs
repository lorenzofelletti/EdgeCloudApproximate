#[derive(Debug, Clone, Copy)]
/// Enum of the possible strategies to send messages to Kafka partitions.
pub enum SendStrategy {
    /// Send messages to a random partition.
    Random,
    /// Send messages to partitions in a round-robin fashion.
    RoundRobin,
}

#[derive(Debug, Clone, Copy)]
pub enum SamplingStrategy {
    /// Sample messages randomly.
    Random,
    /// Sample messages using stratified sampling.
    Stratified,
    ///// Discard all messages when the message count exceed a thresho the message
    ///// count exceed a threshold
    //MsgCountThreshold,
}
