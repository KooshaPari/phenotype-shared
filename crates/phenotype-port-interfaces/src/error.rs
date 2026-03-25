//! # Error Types
//!
//! Common error types used across ports.

use thiserror::Error;

/// Common error type for port operations.
#[derive(Error, Debug)]
pub enum PortError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<std::string::FromUtf8Error> for PortError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        PortError::InvalidData(e.to_string())
    }
}

/// Result type alias for port operations.
pub type Result<T> = std::result::Result<T, PortError>;
