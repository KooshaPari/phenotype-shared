//! # Policy ID

use crate::errors::ValidationError;

/// Unique identifier for a policy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyId(String);

impl PolicyId {
    /// Creates a new random PolicyId.
    pub fn new() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let rand = (ts & 0xFFFF_FFFF) ^ (ts >> 17) ^ 0x9ABC;
        Self(format!("pol{:013x}{:011x}", ts, rand))
    }

    /// Parses a PolicyId from a string.
    pub fn parse(s: &str) -> Result<Self, ValidationError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ValidationError::new("PolicyId", "cannot be empty"));
        }
        if s.len() > 32 {
            return Err(ValidationError::new("PolicyId", "exceeds 32 characters"));
        }
        Ok(Self(s.to_string()))
    }

    /// Returns the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PolicyId {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for PolicyId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PolicyId> for String {
    fn from(id: PolicyId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = PolicyId::new();
        assert!(id.as_str().starts_with("pol"));
    }

    #[test]
    fn test_parse() {
        let id = PolicyId::parse("pol123").unwrap();
        assert_eq!(id.as_str(), "pol123");
    }
}
