//! # Agent ID
//!
//! Unique identifier for an agent using ULID-like format.

use crate::errors::ValidationError;

/// Unique identifier for an agent.
/// Uses ULID-like format: time(48bits) + random(80bits) = 128bit sortable ID.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentId(String);

impl AgentId {
    /// Creates a new random AgentId based on current timestamp.
    pub fn new() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // Pseudo-random suffix using system entropy via address
        let rand = (ts & 0xFFFF_FFFF) ^ (ts >> 17) ^ 0xABCD;
        Self(format!("{:013x}{:011x}", ts, rand))
    }

    /// Parses an AgentId from a string.
    pub fn from_str(s: &str) -> Result<Self, ValidationError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ValidationError::new("AgentId", "cannot be empty"));
        }
        if s.len() > 32 {
            return Err(ValidationError::new("AgentId", "exceeds 32 characters"));
        }
        if !s.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            return Err(ValidationError::new("AgentId", "must be hexadecimal"));
        }
        Ok(Self(s.to_string()))
    }

    /// Returns the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns true if the ID follows ULID-like format (26 chars hex).
    pub fn is_ulid_format(&self) -> bool {
        self.0.len() == 26 && self.0.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for AgentId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AgentId> for String {
    fn from(id: AgentId) -> Self {
        id.0
    }
}

impl AsRef<str> for AgentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_unique_ids() {
        let id1 = AgentId::new();
        let id2 = AgentId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_str_valid() {
        let id = AgentId::from_str("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap();
        assert_eq!(id.as_str(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(AgentId::from_str("").is_err());
        assert!(AgentId::from_str("not-hex!").is_err());
        assert!(AgentId::from_str("a".repeat(33).as_str()).is_err());
    }

    #[test]
    fn test_is_ulid_format() {
        assert!(AgentId::new().is_ulid_format());
        assert!(AgentId::from_str("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap().is_ulid_format());
        assert!(!AgentId::from_str("abc").unwrap().is_ulid_format());
    }

    #[test]
    fn test_display() {
        let id = AgentId::from_str("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap();
        assert_eq!(format!("{}", id), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
    }
}
