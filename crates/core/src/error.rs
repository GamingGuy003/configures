#[derive(Debug, Clone)]
pub enum ConfiguresError {
    FileManipulationError(FileManipulationError),
    GitError(GitError),
    StorageError(StorageError),
    BuilderError(BuilderError),
}

#[derive(Debug, Clone)]
pub enum FileManipulationError {
    LinkingError(String),
    NoEscalationTool,
    PermissionDenied,
    ExecutionError(String),
}

#[derive(Debug, Clone)]
pub enum GitError {
    CloneFailed(String),
}

#[derive(Debug, Clone)]
pub enum StorageError {
    NoLocalFolder,
    CreationFailed(String),
}

#[derive(Debug, Clone)]
pub enum BuilderError {
    MissingRemote,
    MissingLocalFolder,
}
