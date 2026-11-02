use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use crate::error::{BuilderError, ConfigError, ConfiguresError};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    remote_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigBuilder {
    remote_url: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self { remote_url: None }
    }

    pub fn remote_url(mut self, remote_url: impl Into<String>) -> Self {
        self.remote_url = Some(remote_url.into());
        self
    }

    pub fn build(self) -> Result<Config, ConfiguresError> {
        if let Some(remote_url) = self.remote_url {
            Ok(Config { remote_url })
        } else {
            Err(BuilderError::MissingRemote.into())
        }
    }
}

impl Config {
    /// attempts to load the config file from disk
    pub fn load(config_path: impl Into<PathBuf>) -> Result<Self, ConfiguresError> {
        let path = config_path.into();
        // if the path exists build config from read file
        if path.exists() {
            let buffer = std::fs::read(path)
                .map_err(|_| ConfiguresError::ConfigError(ConfigError::ReadingError))?;
            let config = toml::from_slice::<ConfigBuilder>(&buffer).map_err(|err| {
                ConfiguresError::ConfigError(ConfigError::ParsingError(err.to_string()))
            })?;

            return config.build();
        }
        // if no config path exists perform initial setup
        let mut remote_url = String::new();
        let _ = std::io::stdin()
            .read_line(&mut remote_url)
            .map_err(|_| ConfigError::IOError)?;
        println!("{remote_url}");
        Ok(Self { remote_url })
    }

    pub fn get_remote(self) -> String {
        self.remote_url
    }
}
