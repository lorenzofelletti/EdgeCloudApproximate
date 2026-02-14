use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Struct holding the message Kafka message that will be sent.
pub struct Message {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub time: String,
    pub speed: f64,
}

impl Message {
    /// Serialize the `Message` in JSON format.
    pub fn json_serialize(&self) -> Value {
        json!({
            "id": self.id,
            "lat": self.lat,
            "lon": self.lon,
            "time": self.time,
            "speed": self.speed,
        })
    }
}
