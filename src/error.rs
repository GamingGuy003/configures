use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to apply profile: {0}")]
    ProfileApply(std::io::Error),
    #[error("Failed to link {0}: {1}")]
    LinkError(std::path::PathBuf, std::io::Error),
    #[error("Failed to unlink {0}: {1}")]
    UnlinkError(std::path::PathBuf, std::io::Error),
    #[error("Failed top copy {0}: {1}")]
    CopyError(std::path::PathBuf, std::io::Error),
}
