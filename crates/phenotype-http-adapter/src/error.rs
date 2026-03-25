//! # Error Types

use thiserror::Error;

/// Error type for HTTP adapter operations.
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Response error: {0}")]
    ResponseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Status code error: {status} - {body}")]
    StatusError { status: u16, body: String },
}

/// Result type alias for HTTP adapter operations.
pub type Result<T> = std::result::Result<T, HttpError>;
