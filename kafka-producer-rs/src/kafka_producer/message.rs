use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub static KEY: RwLock<String> = RwLock::new(String::new());

#[derive(Debug, Clone, Copy, Default)]
pub enum MessageType {
    #[default]
    Message,
    Value,
}

impl<'a, T> From<T> for MessageType
where
    T: Into<&'a str>,
{
    fn from(value: T) -> Self {
        match value.into() {
            "value" => MessageType::Value,
            _ => MessageType::Message,
        }
    }
}

pub trait KafkaMessage {
    fn json_serialize(self) -> Value;
    fn key(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Struct holding the message Kafka message that will be sent.
pub struct Message {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub time: String,
    pub speed: f64,
}

impl KafkaMessage for Message {
    /// Serialize the `Message` in JSON format.
    fn json_serialize(self) -> Value {
        json!({
            "id": self.id,
            "lat": self.lat,
            "lon": self.lon,
            "time": self.time,
            "speed": self.speed,
        })
    }

    fn key(&self) -> String {
        self.id.clone()
    }
}

// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
// pub struct KValue {
//     pub value: Value,
//     pub key: String,
// }

// impl KafkaMessage for KValue {
//     fn json_serialize(self) -> Value {
//         self.value
//     }

//     fn key(&self) -> String {
//         self.value[self.key.clone()]
//             .as_str()
//             .unwrap_or("None")
//             .to_string()
//     }
// }

// impl KafkaMessage for Value {
//     fn json_serialize(self) -> Value {
//         self
//     }

//     fn key(&self) -> String {
//         self["id"].as_str().unwrap_or("None").to_string()
//     }
// }

impl KafkaMessage for Value {
    fn json_serialize(self) -> Value {
        self
    }

    fn key(&self) -> String {
        let key = KEY.read().unwrap();
        let keystr = key.as_str();
        self[keystr].as_str().unwrap_or("None").to_string()
    }
}
