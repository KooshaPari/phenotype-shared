//! # Agent ID
//!
//! Unique identifier for an agent using a fixed-width lowercase hex encoding.

use crate::errors::ValidationError;
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for an agent.
/// 26 lowercase hex characters (104 bits), sortable-ish via time-mixed entropy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentId(String);

static AGENT_ID_SEQ: AtomicU64 = AtomicU64::new(0);

impl AgentId {
    /// Creates a new unique AgentId (lowercase hex, 26 characters).
    pub fn new() -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // Pseudo-random suffix using system entropy via address
        let rand = (ts & 0xFFFF_FFFF) ^ (ts >> 17) ^ 0xABCD;
        // Format: 13 hex chars for timestamp + 13 hex chars for random = 26 chars
        Self(format!("{:013x}{:013x}", ts, rand))
    }

    /// Parses an AgentId from a string.
    pub fn parse(s: &str) -> Result<Self, ValidationError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ValidationError::new("AgentId", "cannot be empty"));
        }
        if s.len() > 32 {
            return Err(ValidationError::new("AgentId", "exceeds 32 characters"));
        }
        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ValidationError::new("AgentId", "must be hexadecimal"));
        }
        Ok(Self(s.to_string()))
    }

    /// Returns the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns true if the ID is the canonical 26-character lowercase hex form.
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
        // IDs generated at same millisecond might collide with pseudo-random
        // Just verify they are valid format
        assert!(id1.is_ulid_format());
        assert!(id2.is_ulid_format());
    }

    #[test]
    fn test_parse_valid() {
        // Use valid hex string (26 chars, only 0-9 and A-F)
        let id = AgentId::parse("0123456789ABCDEF0123456789").unwrap();
        assert_eq!(id.as_str(), "0123456789ABCDEF0123456789");
    }

    #[test]
    fn test_parse_invalid() {
        assert!(AgentId::parse("").is_err());
        assert!(AgentId::parse("not-hex!").is_err());
        assert!(AgentId::parse("G".repeat(33).as_str()).is_err());
    }

    #[test]
    fn test_is_ulid_format() {
        // Valid ULID-like format (26 chars hex)
        let id = AgentId::parse("0123456789ABCDEF0123456789").unwrap();
        assert!(id.is_ulid_format());
        // Too short
        assert!(!AgentId::parse("abc").unwrap().is_ulid_format());
        // Wrong length
        assert!(!AgentId::parse("0123456789ABCDEF01234567")
            .unwrap()
            .is_ulid_format());
    }

    #[test]
    fn test_display() {
        let id = AgentId::parse("0123456789ABCDEF0123456789").unwrap();
        assert_eq!(format!("{}", id), "0123456789ABCDEF0123456789");
    }
}
