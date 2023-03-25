use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Message {
    id: String,
    lat: f64,
    lon: f64,
    time: String,
    speed: f64,
}

impl Message {
    fn new(id: String, lat: f64, lon: f64, time: String, speed: f64) -> Self {
        Message {
            id,
            lat,
            lon,
            time,
            speed,
        }
    }
    fn new_from_string_record() -> () {}
}