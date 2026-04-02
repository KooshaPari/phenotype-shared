//! # Workflow Name

use crate::errors::ValidationError;

/// Human-readable name for a workflow.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkflowName(String);

impl WorkflowName {
    /// Creates a new WorkflowName after validation.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_string();
        if s.is_empty() {
            return Err(ValidationError::new("WorkflowName", "cannot be empty"));
        }
        if s.len() > 256 {
            return Err(ValidationError::new(
                "WorkflowName",
                "exceeds 256 characters",
            ));
        }
        Ok(Self(s))
    }

    /// Returns the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for WorkflowName {
    fn default() -> Self {
        Self("Unnamed Workflow".into())
    }
}

impl From<WorkflowName> for String {
    fn from(name: WorkflowName) -> Self {
        name.0
    }
}

impl core::fmt::Display for WorkflowName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = WorkflowName::new("Build and Test").unwrap();
        assert_eq!(name.as_str(), "Build and Test");
    }

    #[test]
    fn test_empty_fails() {
        assert!(WorkflowName::new("").is_err());
    }
}
