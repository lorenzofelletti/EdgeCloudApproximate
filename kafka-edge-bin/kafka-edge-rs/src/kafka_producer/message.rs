use geohash::{Coord, GeohashError};
use geojson::Feature;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

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

    /// Deserialize a JSON `Message`.
    pub fn json_deserialize(message: Value) -> Result<Message, Error> {
        serde_json::from_value(message)
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

    /// Get the neighborhood of the message.
    pub fn get_neighborhood(
        &self,
        features: &Vec<Feature>
    ) -> Option<Feature> {
        let geohash = self.geohash();
        if geohash.is_err() {
            return None;
        }
        let geohash = geohash.unwrap();

        for feature in features {
            feature.properties.unwrap().get("NAME");
            let neighborhood_geohash = feature
                .properties
                .get("geohash")
                .expect("No geohash found for neighborhood!");
            let neighborhood_geohash = neighborhood_geohash.as_str().unwrap();

            if geohash.starts_with(neighborhood_geohash) {
                return Some(feature.clone());
            }
        }
        
        None
}
