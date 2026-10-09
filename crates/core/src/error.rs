#[derive(Debug, Clone)]
pub enum ConfiguresError {
    FileManipulationError(FileManipulationError),
    GitError(GitError),
    StorageError(StorageError),
}

#[derive(Debug, Clone)]
pub enum FileManipulationError {
    LinkingError(String),
    NoEscalationTool,
    PermissionDenied,
    ExecutionError(String),
}

#[derive(Debug, Clone)]
pub struct GitError;

#[derive(Debug, Clone)]
pub struct StorageError;
