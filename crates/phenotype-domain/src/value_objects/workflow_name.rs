//! # Workflow Name
//!
//! Human-readable name for a workflow.

use crate::errors::ValidationError;
use crate::value_objects::ValueObject;

/// Human-readable name for a workflow.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkflowName(String);

impl WorkflowName {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_string();
        if s.is_empty() {
            return Err(ValidationError::new("WorkflowName", "cannot be empty"));
        }
        if s.len() > 256 {
            return Err(ValidationError::new("WorkflowName", "cannot exceed 256 characters"));
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for WorkflowName {
    fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        Self::new(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for WorkflowName {
    fn default() -> Self {
        Self("Unnamed Workflow".to_string())
    }
}

impl From<WorkflowName> for String {
    fn from(name: WorkflowName) -> Self {
        name.0
    }
}

impl std::fmt::Display for WorkflowName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_name_creation() {
        let name = WorkflowName::new("CI Pipeline").unwrap();
        assert_eq!(name.as_str(), "CI Pipeline");
    }
}
