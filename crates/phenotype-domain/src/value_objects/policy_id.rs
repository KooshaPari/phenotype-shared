//! # Policy ID
//!
//! Unique identifier for a policy.

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Unique identifier for a policy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyId(String);

impl PolicyId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_lowercase();
        if s.is_empty() {
            return Err(ValidationError::new("PolicyId", "cannot be empty"));
        }
        if s.len() > 128 {
            return Err(ValidationError::new("PolicyId", "cannot exceed 128 characters"));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for PolicyId {
    fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        Self::new(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PolicyId {
    fn default() -> Self {
        Self("policy-000".to_string())
    }
}

impl From<PolicyId> for String {
    fn from(id: PolicyId) -> Self {
        id.0
    }
}

impl std::fmt::Display for PolicyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_id_creation() {
        let id = PolicyId::new("POLICY-001").unwrap();
        assert_eq!(id.as_str(), "policy-001");
    }
}
