use serde_derive::{Deserialize, Serialize};

use crate::error::ConfiguresError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    remote_url: Option<String>,
}

impl Config {
    // attempts to load the config file from disk
    pub fn load() -> Result<Self, ConfiguresError> {
        Ok(Self { remote_url: None })
    }
}
