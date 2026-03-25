//! # Error Types

use thiserror::Error;

/// Error type for Postgres adapter operations.
#[derive(Error, Debug)]
pub enum PostgresError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Pool error: {0}")]
    PoolError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),
}

/// Result type alias for Postgres adapter operations.
pub type Result<T> = std::result::Result<T, PostgresError>;
