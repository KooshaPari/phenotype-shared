//! # Task Name

use crate::errors::ValidationError;

/// Human-readable name for a task.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskName(String);

impl TaskName {
    /// Creates a new TaskName after validation.
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into().trim().to_string();
        if s.is_empty() {
            return Err(ValidationError::new("TaskName", "cannot be empty"));
        }
        if s.len() > 256 {
            return Err(ValidationError::new("TaskName", "exceeds 256 characters"));
        }
        Ok(Self(s))
    }

    /// Returns the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TaskName {
    fn default() -> Self {
        Self("Untitled Task".into())
    }
}

impl From<TaskName> for String {
    fn from(name: TaskName) -> Self {
        name.0
    }
}

impl core::fmt::Display for TaskName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = TaskName::new("Build Project").unwrap();
        assert_eq!(name.as_str(), "Build Project");
    }

    #[test]
    fn test_empty_fails() {
        assert!(TaskName::new("").is_err());
    }

    #[test]
    fn test_trimmed() {
        let name = TaskName::new("  Test  ").unwrap();
        assert_eq!(name.as_str(), "Test");
    }
}
