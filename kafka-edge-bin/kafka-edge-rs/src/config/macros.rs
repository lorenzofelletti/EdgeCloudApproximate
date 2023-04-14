#[macro_export]
macro_rules! read_duration_key_from_table {
    ($table:expr, $key:expr, $data:expr) => {
        Duration::from_millis(
            read_integer_key_from_table($table, $key, &$data)?
                .try_into()
                .unwrap_or_default(),
        )
    };
}

#[macro_export]
macro_rules! read_duration {
    ($name:ident, $table:expr, $data:expr) => {
        let $name = read_duration_key_from_table!($table, stringify!($name), $data);
    };
}

#[macro_export]
macro_rules! invalid_value_err {
    ($key:ident) => {
        Err(ConfigurationError::new(
            $key,
            ErrorType::InvalidValueForKey(stringify!($key).to_owned()),
        ))
    };
}

#[macro_export]
macro_rules! read_string_with_match {
    ($key:ident, $table_name:expr, $data:expr, $($matcher:pat => $result:expr),*) => {
        let $key = read_string_key_from_table($table_name, stringify!($key), &$data)?;
        let $key = match $key.to_lowercase().as_str() {
            $($matcher => $result,)*
            _ => invalid_value_err!($key)
        }?;
    };
}

#[macro_export]
macro_rules! read_array_of_hosts {
    ($key:ident, $table_name:expr, $data:expr) => {
        let $key = read_array_key_from_table($table_name, stringify!($key), &$data)?;
        let $key: Vec<String> = from_vec_of_value_to_vec_of_string($key);
        check_value_not_empty_or_has_empty_strings(&$key, stringify!($key))?;
    };
}

#[macro_export]
macro_rules! read_path {
    ($key:ident, $table_name:expr, $data:expr) => {
        PathBuf::from(read_string_key_from_table(
            $table_name,
            stringify!($key),
            &$data,
        )?)
    };
}
