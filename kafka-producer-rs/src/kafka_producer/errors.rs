use std::{
    error::Error,
    fmt::{self, Debug},
};

#[derive(Debug, PartialEq)]
pub enum KafkaSendError {
    ProducerCreationError(String),
    UnknownError(String),
}

impl KafkaSendError {
    pub fn new_unknown_error<S: Into<String>>(message: S) -> KafkaSendError {
        KafkaSendError::UnknownError(message.into())
    }
}

impl Error for KafkaSendError {}

impl fmt::Display for KafkaSendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownError(topic) => {
                write!(f, "Error sending message to Kafka topic '{topic}'.")
            }
            Self::ProducerCreationError(hosts) => {
                write!(f, "Error creating the producer from hosts '{hosts}'")
            }
        }
    }
}
