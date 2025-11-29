use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to load profile files: {0}")]
    ProfileLoad(std::io::Error),
    #[error("Failed to read metadata {0}: {1}")]
    ConfigRead(std::path::PathBuf, std::io::Error),
    #[error("Failed to write metadata {0}: {1}")]
    ConfigWrite(std::path::PathBuf, std::io::Error),
    #[error("Failed to parse metadata: {0}")]
    ConfigParse(serde_json::Error),
    #[error("Failed to serialize metadata: {0}")]
    ConfigSerialize(serde_json::Error),
    #[error("Failed to apply profile: {0}")]
    ProfileApply(std::io::Error),
    #[error("Failed to link {0}: {1}")]
    Link(std::path::PathBuf, std::io::Error),
    #[error("Failed to unlink {0}: {1}")]
    Unlink(std::path::PathBuf, std::io::Error),
    #[error("Failed top copy {0}: {1}")]
    Copy(std::path::PathBuf, std::io::Error),
    #[error("Failed to calculate timestamp: {0}")]
    SystemTime(std::time::SystemTimeError),
}
