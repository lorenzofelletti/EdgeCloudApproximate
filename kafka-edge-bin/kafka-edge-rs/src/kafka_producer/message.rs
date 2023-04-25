use geohash::{Coord, GeohashError};

use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Struct holding the incoming Kafka message.
pub struct Message {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub time: String,
    pub speed: f64,
    /// The geohash of the message.
    pub geohash: Option<String>,
    /// The neighborhood of the message.
    pub neighborhood: Option<String>,
}

impl Message {
    /// Serialize the `Message` in JSON format.
    ///
    /// # Panics
    /// Panics if the message does not have a geohash and a neighborhood.
    pub fn json_serialize(&self) -> Value {
        json!({
            "id": self.id,
            "lat": self.lat,
            "lon": self.lon,
            "time": self.time,
            "speed": self.speed,
            "geohash": self.geohash.as_ref().unwrap(),
            "neighborhood": self.neighborhood.as_ref().unwrap(),
        })
    }

    /// Deserialize a JSON `Message`.
    pub fn json_deserialize(message: &[u8]) -> Result<Message, Error> {
        serde_json::from_slice(message)
    }

    /// Calculate the geohash of the message.
    pub fn geohash(&self) -> Result<String, GeohashError> {
        geohash::encode(
            Coord {
                x: self.lat,
                y: self.lon,
            },
            6,
        )
    }

    /// Compare the geohash of the message with the geohash of another message.
    ///
    /// # Panics
    /// Panics if one of the messages does not have a geohash.
    pub fn geohash_cmp(&self, other: &Message) -> std::cmp::Ordering {
        self.geohash
            .as_ref()
            .unwrap()
            .cmp(&other.geohash.as_ref().unwrap())
    }
}
