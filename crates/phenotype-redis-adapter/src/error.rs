//! Error types for the redis adapter.

use thiserror::Error;
use phenotype_port_interfaces::error::PortError;

/// Adapter-specific error type.
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Redis error: {0}")]
    Redis(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Port error: {0}")]
    Port(#[from] PortError),
}

/// Result type alias for adapter operations.
pub type Result<T> = std::result::Result<T, AdapterError>;
