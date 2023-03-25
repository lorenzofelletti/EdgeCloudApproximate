use std::{env, error::Error, path::PathBuf};

use super::toml_template::TOML_FILE_NAME;

pub fn get_zookeeper_string(zookeeper: &Vec<String>) -> String {
    zookeeper.join(",")
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // Get current executable path
    let executable_path = env::current_exe()?;

    let mut output_path = PathBuf::from(executable_path);
    output_path.set_file_name(TOML_FILE_NAME);

    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zookeeper_string_with_more_entries() {
        let zookeeper_arr = vec![
            "192.168.56.10:2181".to_owned(),
            "192.168.56.11:2181".to_owned(),
        ];
        assert_eq!(
            get_zookeeper_string(&zookeeper_arr),
            "192.168.56.10:2181,192.168.56.11:2181"
        )
    }

    #[test]
    fn test_zookeeper_string_with_one_entry() {
        let zookeeper_arr = vec!["192.168.56.10:2181".to_owned()];
        assert_eq!(get_zookeeper_string(&zookeeper_arr), "192.168.56.10:2181")
    }
}
