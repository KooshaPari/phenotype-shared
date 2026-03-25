//! # Agent ID

use crate::errors::ValidationError;

/// Unique identifier for an agent.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentId(String);

impl AgentId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_lowercase();
        if s.is_empty() {
            return Err(ValidationError::new("AgentId", "cannot be empty"));
        }
        if s.len() > 128 {
            return Err(ValidationError::new("AgentId", "cannot exceed 128 chars"));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self("agent-000".into())
    }
}

impl From<AgentId> for String {
    fn from(id: AgentId) -> Self {
        id.0
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id() {
        let id = AgentId::new("AGENT-001").unwrap();
        assert_eq!(id.as_str(), "agent-001");
    }

    #[test]
    fn test_agent_id_empty_fails() {
        assert!(AgentId::new("").is_err());
    }
}
