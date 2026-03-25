//! Error types for the postgres adapter.

use thiserror::Error;
use phenotype_port_interfaces::error::PortError;

/// Adapter-specific error type.
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Postgres error: {0}")]
    Postgres(String),

    #[error("Pool error: {0}")]
    Pool(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Port error: {0}")]
    Port(#[from] PortError),
}

/// Result type alias for adapter operations.
pub type Result<T> = std::result::Result<T, AdapterError>;
