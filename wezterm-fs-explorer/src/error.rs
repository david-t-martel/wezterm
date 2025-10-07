use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ExplorerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
}