//! # Agent Name

use crate::errors::ValidationError;

/// Human-readable name for an agent.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AgentName(String);

impl AgentName {
    /// Creates a new AgentName after validation.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_string();
        if s.is_empty() {
            return Err(ValidationError::new("AgentName", "cannot be empty"));
        }
        if s.len() > 256 {
            return Err(ValidationError::new("AgentName", "exceeds 256 characters"));
        }
        Ok(Self(s))
    }

    /// Creates an AgentName without validation (for trusted input).
    pub fn from_trusted(value: String) -> Self {
        Self(value)
    }

    /// Returns the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for AgentName {
    fn default() -> Self {
        Self("Unnamed Agent".into())
    }
}

impl From<AgentName> for String {
    fn from(name: AgentName) -> Self {
        name.0
    }
}

impl core::fmt::Display for AgentName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AgentName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = AgentName::new("Test Agent").unwrap();
        assert_eq!(name.as_str(), "Test Agent");
    }

    #[test]
    fn test_empty_name_fails() {
        assert!(AgentName::new("").is_err());
    }

    #[test]
    fn test_whitespace_trimmed() {
        let name = AgentName::new("  Agent  ").unwrap();
        assert_eq!(name.as_str(), "Agent");
    }

    #[test]
    fn test_long_name_fails() {
        let long_name = "a".repeat(257);
        assert!(AgentName::new(long_name.as_str()).is_err());
    }

    #[test]
    fn test_default() {
        let name = AgentName::default();
        assert_eq!(name.as_str(), "Unnamed Agent");
    }
}
