use std::collections::HashMap;

#[macro_export]
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                warn!("An error: {}; skipped.", e);
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! skip_none {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => {
                warn!("Skipped on a None value.");
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! create_record {
    ($topic:expr, $msg:expr) => {
        Record::from_key_value(&$topic, $msg.id.clone(), $msg.json_serialize().to_string())
    };
}

#[macro_export]
macro_rules! create_record_with_partition {
    ($topic:expr, $msg:expr, $partition:expr) => {
        Record::from_key_value(&$topic, $msg.id.clone(), $msg.json_serialize().to_string())
            .with_partition($partition)
    };
}

pub fn get_msg_neighborhood(
    msg_gh: &String,
    neighborhood_geohashes: &HashMap<String, Vec<String>>,
) -> Option<String> {

    // find key that contains the geohash in its value vector
    neighborhood_geohashes.iter().find_map(|(key, &ref val)| {
        if val.iter().any(|gh| gh == msg_gh) {
            Some(key)
        } else {
            None
        }
    }).cloned()
}
