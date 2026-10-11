use std::path::Path;

use directories::BaseDirs;

use crate::{
    error::{BuilderError, ConfiguresError, StorageError},
    git::clone_remote,
};

pub mod error;
pub mod file_operations;
pub mod git;
pub mod storage;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ConfiguresCore {
    remote_url: String,
    local_path: String,
}

impl ConfiguresCore {
    /// initializes the git repo locally based off of remote
    pub fn init(&self) -> Result<(), ConfiguresError> {
        clone_remote(self.remote_url.clone(), Path::new(&self.local_path))
    }
}

#[derive(Default)]
pub struct ConfiguresCoreBuilder {
    remote_url: Option<String>,
}

impl ConfiguresCoreBuilder {
    pub fn new() -> Self {
        Self { remote_url: None }
    }

    pub fn remote_url(mut self, remote_url: impl Into<String>) -> Self {
        self.remote_url = Some(remote_url.into());
        self
    }

    /// tries to build an instance of ConfiguresCore with the arguments supplied by the builder
    /// functions; Also attempts to figure out where to store local data
    pub fn build(self) -> Result<ConfiguresCore, ConfiguresError> {
        // should result in ~/.configures
        let local_path = BaseDirs::new()
            .ok_or(ConfiguresError::StorageError(StorageError::NoLocalFolder))?
            .home_dir()
            .join(".configures")
            .to_str()
            .ok_or(ConfiguresError::BuilderError(
                BuilderError::MissingLocalFolder,
            ))?
            .to_string();
        Ok(ConfiguresCore {
            remote_url: self
                .remote_url
                .ok_or(ConfiguresError::BuilderError(BuilderError::MissingRemote))?,
            local_path,
        })
    }
}
