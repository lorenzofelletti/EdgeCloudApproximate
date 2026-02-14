use std::sync::OnceLock;

use geohash::{Coord, GeohashError};

use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Number, Value};

pub static LAT_KEY: OnceLock<usize> = OnceLock::new();
pub static LON_KEY: OnceLock<usize> = OnceLock::new();

// lazy_static! {
//     pub static ref LAT_KEY: RwLock<usize> = RwLock::new(1);
//     pub static ref LON_KEY: RwLock<usize> = RwLock::new(2);
// }

pub trait GeoMessage {
    fn geohash(&self) -> Result<String, GeohashError>;
    fn set_geohash(&mut self, geohash: String);
}

pub trait WithNeighborhood {
    fn set_neighborhood(&mut self, neighborhood: String);
    fn neighborhood(&self) -> Option<String>;
}

pub trait JSONMessage: JSONMessageDeserialize + JSONMessageSerialize {}

pub trait JSONMessageSerialize
where
    Self: Sized,
{
    fn json_serialize(&self) -> Value;
}

pub trait JSONMessageDeserialize
where
    Self: Sized,
{
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

    fn set_geohash(&mut self, geohash: String) {
        self.geohash = Some(geohash);
    }
}

impl WithNeighborhood for Message {
    fn set_neighborhood(&mut self, neighborhood: String) {
        self.neighborhood = Some(neighborhood);
    }

    fn neighborhood(&self) -> Option<String> {
        return self.neighborhood.clone();
    }
}

impl JSONMessageSerialize for Message {
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
}

impl JSONMessageDeserialize for Message {
    /// Deserialize a JSON `Message`.
    fn json_deserialize(message: &[u8]) -> Result<Message, Error> {
        serde_json::from_slice(message)
    }
}

impl JSONMessage for Message {}

impl GeoMessage for Vec<Value> {
    fn geohash(&self) -> Result<String, GeohashError> {
        let minus_one = Number::from_f64(-1.0).unwrap();

        let lat_key = LAT_KEY.get().unwrap().clone();
        let lon_key = LON_KEY.get().unwrap().clone();
        let lat = self
            .get(lat_key)
            .unwrap_or(&Value::Null)
            .as_number()
            .unwrap_or(&minus_one)
            .as_f64()
            .unwrap_or(-1.0);
        let lon = self
            .get(lon_key)
            .unwrap_or(&Value::Null)
            .as_number()
            .unwrap_or(&minus_one)
            .as_f64()
            .unwrap_or(-1.0);

        // println!("lat: {}", lat);
        // println!("lon: {}", lon);

        geohash::encode(Coord { x: lat, y: lon }, 6)
    }

    /// Sets the geohash of the message.
    fn set_geohash(&mut self, geohash: String) {
        self.push(Value::String(geohash));
    }
}

impl WithNeighborhood for Vec<Value> {
    fn set_neighborhood(&mut self, neighborhood: String) {
        self.push(Value::String(neighborhood));
    }

    fn neighborhood(&self) -> Option<String> {
        if self.len() == 1 {
            None
        } else {
            self.get(self.len() - 1)
                .map(|v| v.as_str().unwrap_or_default().to_string())
        }
    }
}

impl JSONMessageSerialize for Vec<Value> {
    fn json_serialize(&self) -> Value {
        serde_json::to_value(self).expect("couldn't serialize")
    }
}

impl JSONMessageDeserialize for Vec<Value> {
    fn json_deserialize(message: &[u8]) -> Result<Self, Error> {
        serde_json::from_slice(message)
    }
}

impl JSONMessage for Vec<Value> {}
