use geohash::{Coord, GeohashError};

use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

pub trait GeoMessage {
    fn geohash(&self) -> Result<String, GeohashError>;
}

pub trait JSONMessage
where
    Self: Sized,
{
    fn json_serialize(&self) -> Value;
    fn json_deserialize(message: &[u8]) -> Result<Self, Error>;
}

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

impl GeoMessage for Message {
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

impl JSONMessage for Message {
    /// Serialize the `Message` in JSON format.
    ///
    /// # Panics
    /// Panics if the message does not have a geohash and a neighborhood.
    fn json_serialize(&self) -> Value {
        json!({
            "id": self.id,
            "lat": self.lat,
            "lon": self.lon,
            "time": self.time,
            "speed": self.speed,
            "geohash": self.geohash.as_ref().unwrap(),
            "neighborhood": self.neighborhood.as_ref().unwrap_or(&String::from("")),
        })
    }

    /// Deserialize a JSON `Message`.
    fn json_deserialize(message: &[u8]) -> Result<Message, Error> {
        serde_json::from_slice(message)
    }
}

impl GeoMessage for Value {
    fn geohash(&self) -> Result<String, GeohashError> {
        todo!()
    }
}

impl JSONMessage for Value {
    fn json_serialize(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }

    fn json_deserialize(message: &[u8]) -> Result<Self, Error> {
        serde_json::from_slice(message)
    }
}
