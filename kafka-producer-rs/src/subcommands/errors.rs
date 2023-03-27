use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct SubcommandError {
    message: String,
}

impl Error for SubcommandError {}

impl fmt::Display for SubcommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {0}", self.message)
    }
}

impl SubcommandError {
    pub fn new<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        SubcommandError {
            message: message.into(),
        }
    }
}
