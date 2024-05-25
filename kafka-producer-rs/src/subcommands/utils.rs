/// Joins a `Vec<String>` by comma except for the last element, that will not
/// end with a comma.
pub fn join_by_comma(zookeeper: &[String]) -> String {
    zookeeper.join(",")
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
            join_by_comma(&zookeeper_arr),
            "192.168.56.10:2181,192.168.56.11:2181"
        )
    }

    #[test]
    fn test_zookeeper_string_with_one_entry() {
        let zookeeper_arr = vec!["192.168.56.10:2181".to_owned()];
        assert_eq!(join_by_comma(&zookeeper_arr), "192.168.56.10:2181")
    }
}
