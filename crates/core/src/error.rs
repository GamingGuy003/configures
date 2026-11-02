#[derive(Debug, Clone)]
pub enum ConfiguresError {
    FileManipulationError(FileManipulationError),
    GitError(GitError),
    StorageError(StorageError),
    BuilderError(BuilderError),
    ConfigError(ConfigError),
}

#[derive(Debug, Clone)]
pub enum FileManipulationError {
    LinkingError(String),
    NoEscalationTool,
    PermissionDenied,
    ExecutionError(String),
}

impl From<FileManipulationError> for ConfiguresError {
    fn from(value: FileManipulationError) -> Self {
        Self::FileManipulationError(value)
    }
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    ReadingError,
    WritingError,
    ParsingError(String),
    IOError,
}

impl From<ConfigError> for ConfiguresError {
    fn from(value: ConfigError) -> Self {
        Self::ConfigError(value)
    }
}

#[derive(Debug, Clone)]
pub enum GitError {
    CloneFailed(String),
}

impl From<GitError> for ConfiguresError {
    fn from(value: GitError) -> Self {
        Self::GitError(value)
    }
}

#[derive(Debug, Clone)]
pub enum StorageError {
    NoLocalFolder,
    CreationFailed(String),
}

impl From<StorageError> for ConfiguresError {
    fn from(value: StorageError) -> Self {
        Self::StorageError(value)
    }
}

#[derive(Debug, Clone)]
pub enum BuilderError {
    MissingRemote,
    MissingLocalFolder,
}

impl From<BuilderError> for ConfiguresError {
    fn from(value: BuilderError) -> Self {
        ConfiguresError::BuilderError(value)
    }
}
