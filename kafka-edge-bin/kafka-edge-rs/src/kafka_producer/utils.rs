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
