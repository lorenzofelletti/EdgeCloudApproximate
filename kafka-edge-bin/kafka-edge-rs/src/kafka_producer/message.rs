use geohash::{Coord, GeohashError};

use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

pub trait MessageTrait {
    fn json_serialize(&self) -> Value;
    fn json_deserialize(message: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
    fn geohash(&self) -> Result<String, GeohashError>;
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Struct holding the incoming Kafka message.
pub struct Message {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub time: String,
    pub speed: f64,
}

impl MessageTrait for Message {
    /// Serialize the `Message` in JSON format.
    fn json_serialize(&self) -> Value {
        json!({
            "id": self.id,
            "lat": self.lat,
            "lon": self.lon,
            "time": self.time,
            "speed": self.speed,
        })
    }

    /// Deserialize a JSON `Message`.
    fn json_deserialize(message: &[u8]) -> Result<Message, Error> {
        serde_json::from_slice(message)
    }

    /// Calculate the geohash of the message.
    fn geohash(&self) -> Result<String, GeohashError> {
        geohash::encode(
            Coord {
                x: self.lat,
                y: self.lon,
            },
            6,
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Struct holding the Kafka message that will be sent.
pub struct MessageOut {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub time: String,
    pub speed: f64,
    /// The geohash of the message.
    pub geohash: String,
    /// The neighborhood of the message.
    pub neighborhood: String,
}

impl MessageOut {
    /// Create a new `MessageOut` from a `Message`.
    ///
    /// # Arguments
    /// * `message` - The `Message` to be converted.
    /// * `neighborhood` - The neighborhood of the message.
    ///
    /// # Returns
    /// A new `MessageOut` with the same values as the `Message` and the neighborhood.
    ///
    /// # Panics
    /// This function will panic if the geohash of the message cannot be calculated.
    pub fn from_message(message: &Message, neighborhood: String) -> Self {
        Self {
            id: message.id.clone(),
            lat: message.lat,
            lon: message.lon,
            time: message.time.clone(),
            speed: message.speed,
            geohash: message.geohash().unwrap(),
            neighborhood,
        }
    }
}

impl MessageTrait for MessageOut {
    /// Serialize the `Message` in JSON format.
    fn json_serialize(&self) -> Value {
        json!({
            "id": self.id,
            "lat": self.lat,
            "lon": self.lon,
            "time": self.time,
            "speed": self.speed,
            "geohash": self.geohash,
            "neighborhood": self.neighborhood,
        })
    }

    /// Deserialize a JSON `Message`.
    fn json_deserialize(message: &[u8]) -> Result<MessageOut, Error> {
        serde_json::from_slice(message)
    }

    /// Calculate the geohash of the message.
    fn geohash(&self) -> Result<String, GeohashError> {
        geohash::encode(
            Coord {
                x: self.lat,
                y: self.lon,
            },
            6,
        )
    }
}
