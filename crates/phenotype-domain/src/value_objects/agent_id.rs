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
            .as_nanos() as u128;
        let seq = AGENT_ID_SEQ.fetch_add(1, Ordering::Relaxed) as u128;
        let mixed = ts.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(seq);
        let v = mixed & ((1u128 << 104) - 1);
        Self(format!("{:026x}", v))
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
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_str_valid() {
        let sample = "01ab".repeat(7).chars().take(26).collect::<String>();
        let id = AgentId::from_str(&sample).unwrap();
        assert_eq!(id.as_str(), sample.as_str());
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
        let sample = "01ab".repeat(7).chars().take(26).collect::<String>();
        assert!(AgentId::from_str(&sample).unwrap().is_ulid_format());
        assert!(!AgentId::from_str("abc").unwrap().is_ulid_format());
    }

    #[test]
    fn test_display() {
        let sample = "01ab".repeat(7).chars().take(26).collect::<String>();
        let id = AgentId::from_str(&sample).unwrap();
        assert_eq!(format!("{}", id), sample);
    }
}
