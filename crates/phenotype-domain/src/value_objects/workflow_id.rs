//! # Workflow ID

use crate::errors::ValidationError;

/// Unique identifier for a workflow.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowId(String);

impl WorkflowId {
    /// Creates a new random WorkflowId.
    pub fn new() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let rand = (ts & 0xFFFF_FFFF) ^ (ts >> 17) ^ 0x5678;
        Self(format!("wf{:013x}{:011x}", ts, rand))
    }

    /// Parses a WorkflowId from a string.
    pub fn parse(s: &str) -> Result<Self, ValidationError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ValidationError::new("WorkflowId", "cannot be empty"));
        }
        if s.len() > 32 {
            return Err(ValidationError::new("WorkflowId", "exceeds 32 characters"));
        }
        Ok(Self(s.to_string()))
    }

    /// Returns the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for WorkflowId {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for WorkflowId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<WorkflowId> for String {
    fn from(id: WorkflowId) -> Self {
        id.0
    }
}

impl AsRef<str> for WorkflowId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = WorkflowId::new();
        assert!(id.as_str().starts_with("wf"));
    }

    #[test]
    fn test_parse() {
        let id = WorkflowId::parse("wf123").unwrap();
        assert_eq!(id.as_str(), "wf123");
    }
}
