use git2::Repository;
use std::{fs, path::Path};

use crate::error::{ConfiguresError, GitError, StorageError};

/// tries to clone the repository at remote
pub fn clone_remote(
    remote_url: impl Into<String>,
    local_path: &Path,
) -> Result<(), ConfiguresError> {
    // create if not exists
    if !local_path.exists() {
        fs::create_dir_all(local_path).map_err(|err| {
            ConfiguresError::StorageError(StorageError::CreationFailed(err.to_string()))
        })?;
    }

    // try to clone the repo into the local data location
    Repository::clone(&remote_url.into(), local_path)
        .map_err(|err| ConfiguresError::GitError(GitError::CloneFailed(err.to_string())))?;

    Ok(())
}
