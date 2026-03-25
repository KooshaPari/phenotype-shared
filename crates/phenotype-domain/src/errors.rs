//! # Errors
//!
//! Domain error types.

use std::fmt;

/// Result type for domain operations.
pub type DomainResult<T> = Result<T, DomainError>;

/// Domain error types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// Validation failed.
    Validation(ValidationError),
    /// Invalid state transition.
    InvalidStateTransition {
        current: String,
        target: String,
        reason: String,
    },
    /// Operation not allowed in current state.
    InvalidState(String),
    /// Operation not valid.
    InvalidOperation(String),
    /// Entity not found.
    NotFound(String),
    /// Concurrency conflict.
    ConcurrencyConflict {
        expected_version: u64,
        actual_version: u64,
    },
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::Validation(e) => write!(f, "Validation error: {}", e),
            DomainError::InvalidStateTransition { current, target, reason } => {
                write!(f, "Invalid state transition from {} to {}: {}", current, target, reason)
            }
            DomainError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            DomainError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            DomainError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DomainError::ConcurrencyConflict { expected, actual } => {
                write!(f, "Concurrency conflict: expected v{}, got v{}", expected, actual)
            }
        }
    }
}

impl std::error::Error for DomainError {}

/// Validation error details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}
