//! # Domain Errors

use core::fmt;

/// Result type for domain operations.
pub type DomainResult<T> = Result<T, DomainError>;

/// Root domain error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainError {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ErrorKind {
    Validation,
    Invariant,
    NotFound,
    Conflict,
    StateTransition,
    Permission,
}

impl DomainError {
    /// Creates a new validation error.
    pub fn validation(field: &str, reason: &str) -> Self {
        Self {
            kind: ErrorKind::Validation,
            message: format!("{field}: {reason}"),
        }
    }

    /// Creates a new invariant violation error.
    pub fn invariant(rule: &str) -> Self {
        Self {
            kind: ErrorKind::Invariant,
            message: rule.to_string(),
        }
    }

    /// Creates a new not found error.
    pub fn not_found(entity: &str, id: &str) -> Self {
        Self {
            kind: ErrorKind::NotFound,
            message: format!("{entity} not found: {id}"),
        }
    }

    /// Creates a new conflict error.
    pub fn conflict(entity: &str, reason: &str) -> Self {
        Self {
            kind: ErrorKind::Conflict,
            message: format!("{entity} conflict: {reason}"),
        }
    }

    /// Creates a new state transition error.
    pub fn state_transition(from: &str, to: &str) -> Self {
        Self {
            kind: ErrorKind::StateTransition,
            message: format!("invalid state transition: {from} -> {to}"),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<ValidationError> for DomainError {
    fn from(e: ValidationError) -> Self {
        Self {
            kind: ErrorKind::Validation,
            message: e.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Validation Error (for Value Objects)
// ------------------------------------------------------------------------------------------------

/// Validation error for value objects and DTOs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    field: String,
    message: String,
}

impl ValidationError {
    /// Creates a new validation error.
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    /// Returns the field that failed validation.
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Returns the validation message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let e = ValidationError::new("email", "invalid format");
        assert_eq!(e.field(), "email");
        assert_eq!(e.message(), "invalid format");
    }

    #[test]
    fn test_domain_error_kinds() {
        assert_eq!(
            DomainError::validation("field", "required").message(),
            "field: required"
        );
        assert_eq!(
            DomainError::not_found("Agent", "123").message(),
            "Agent not found: 123"
        );
        assert_eq!(
            DomainError::state_transition("Idle", "Running").message(),
            "invalid state transition: Idle -> Running"
        );
    }
}
