//! # Task ID

use crate::errors::ValidationError;

/// Unique identifier for a task.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(String);

impl TaskId {
    /// Creates a new random TaskId.
    pub fn new() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let rand = (ts & 0xFFFF_FFFF) ^ (ts >> 17) ^ 0x1234;
        Self(format!("task{:013x}{:011x}", ts, rand))
    }

    /// Parses a TaskId from a string.
    pub fn from_str(s: &str) -> Result<Self, ValidationError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ValidationError::new("TaskId", "cannot be empty"));
        }
        if s.len() > 32 {
            return Err(ValidationError::new("TaskId", "exceeds 32 characters"));
        }
        Ok(Self(s.to_string()))
    }

    /// Returns the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns true if this is a valid task ID format.
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty() && self.0.len() <= 32
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for TaskId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<TaskId> for String {
    fn from(id: TaskId) -> Self {
        id.0
    }
}

impl AsRef<str> for TaskId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = TaskId::new();
        assert!(id.as_str().starts_with("task"));
    }

    #[test]
    fn test_from_str() {
        let id = TaskId::from_str("task123").unwrap();
        assert_eq!(id.as_str(), "task123");
    }

    #[test]
    fn test_invalid() {
        assert!(TaskId::from_str("").is_err());
    }
}
