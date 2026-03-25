//! # Workflow ID
//!
//! Unique identifier for a workflow.

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Unique identifier for a workflow.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowId(String);

impl WorkflowId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_lowercase();
        if s.is_empty() {
            return Err(ValidationError::new("WorkflowId", "cannot be empty"));
        }
        if s.len() > 128 {
            return Err(ValidationError::new("WorkflowId", "cannot exceed 128 characters"));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for WorkflowId {
    fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        Self::new(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for WorkflowId {
    fn default() -> Self {
        Self("workflow-000".to_string())
    }
}

impl From<WorkflowId> for String {
    fn from(id: WorkflowId) -> Self {
        id.0
    }
}

impl std::fmt::Display for WorkflowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_id_creation() {
        let id = WorkflowId::new("WORKFLOW-001").unwrap();
        assert_eq!(id.as_str(), "workflow-001");
    }
}
