use crate::error::ConfiguresError;

pub mod error;
pub mod file_operations;
pub mod git;
pub mod storage;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ConfiguresCore {
    remote_url: String,
    local_folder: String,
}

#[derive(Default)]
pub struct ConfiguresCoreBuilder {
    remote_url: Option<String>,
    local_folder: Option<String>,
}

impl ConfiguresCoreBuilder {
    pub fn new() -> Self {
        Self {
            remote_url: None,
            local_folder: None,
        }
    }

    pub fn remote_url(mut self, remote_url: impl Into<String>) -> Self {
        self.remote_url = Some(remote_url.into());
        self
    }

    pub fn local_folder(mut self, local_folder: impl Into<String>) -> Self {
        self.local_folder = Some(local_folder.into());
        self
    }

    pub fn build(self) -> Result<ConfiguresCore, ConfiguresError> {
        Ok(ConfiguresCore {
            remote_url: self.remote_url.ok_or(ConfiguresError::BuilderError(
                error::BuilderError::MissingRemote,
            ))?,
            local_folder: self.local_folder.ok_or(ConfiguresError::BuilderError(
                error::BuilderError::MissingLocalFolder,
            ))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_builder_ok() -> Result<(), ConfiguresError> {
        let configures_core = ConfiguresCoreBuilder::new()
            .remote_url("remote_url")
            .local_folder("local_folder")
            .build()?;

        assert_eq!(configures_core.remote_url, "remote_url");
        assert_eq!(configures_core.local_folder, "local_folder");
        Ok(())
    }

    #[test]
    fn test_builder_error() -> Result<(), ConfiguresError> {
        let builder = ConfiguresCoreBuilder::new().remote_url("remote_url");
        assert!(builder.build().is_err());

        let builder = ConfiguresCoreBuilder::new().local_folder("local_folder");
        assert!(builder.build().is_err());

        let builder = ConfiguresCoreBuilder::new()
            .local_folder("local_folder")
            .remote_url("remote_url");
        assert!(builder.build().is_ok());
        Ok(())
    }

    #[test]
    /// create normal symlink
    fn test_create_link_success() -> std::io::Result<()> {
        let dir = tempdir()?;
        let src_path = dir.path().join("source_config");
        let dst_path = dir.path().join("destination_config");

        let mut file = File::create(&src_path)?;
        write!(file, "testcontent")?;

        file_operations::link(&src_path, &dst_path)
            .map_err(|err| std::io::Error::other(format!("{:?}", err)))?;

        assert!(dst_path.exists());
        assert_eq!(fs::read_link(&dst_path)?, src_path);

        let content = fs::read_to_string(dst_path)?;
        assert_eq!(content.trim(), "testcontent");

        Ok(())
    }

    #[test]
    /// create dangling symlink
    fn test_create_dangling_symlink() -> io::Result<()> {
        let dir = tempdir()?;
        let src_path = dir.path().join("source_config");
        let dst_path = dir.path().join("destination_config");

        file_operations::link(&src_path, &dst_path)
            .map_err(|err| std::io::Error::other(format!("{:?}", err)))?;

        assert!(fs::symlink_metadata(&dst_path).is_ok());
        assert!(!dst_path.exists());
        Ok(())
    }

    #[test]
    /// create link to elevated folder
    fn test_build_elevated_command_args() -> io::Result<()> {
        let dir = tempdir()?;
        let src_path = dir.path().join("source_config");
        let dst_path = Path::new("/etc/elevated_config");

        File::create(&src_path)?.write_all(b"testcontent")?;

        file_operations::link(&src_path, dst_path)
            .map_err(|err| std::io::Error::other(format!("{:?}", err)))?;

        assert!(dst_path.exists());
        assert_eq!(fs::read_link(dst_path)?, src_path);

        let content = fs::read_to_string(dst_path)?;
        assert_eq!(content.trim(), "testcontent");

        Ok(())
    }
}
