use thiserror::Error;

#[derive(Error, Debug)]
pub enum OverlordError {
    #[error("Zellij session '{0}' not found")]
    SessionNotFound(String),

    #[error("Zellij session '{0}' already exists")]
    SessionAlreadyExists(String),

    #[error("Failed to execute Zellij command: {0}")]
    ZellijCommandFailed(String),

    #[error("Ritual file not found: {0}")]
    RitualNotFound(String),

    #[error("Layout file not found: {0}")]
    LayoutNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
