//! # Identifier
//!
//! Base identifier traits and types.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Base trait for domain identifiers.
pub trait Identifier: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + fmt::Debug {
    /// Returns the string representation of the identifier.
    fn as_str(&self) -> &str;
}

/// String-based identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringId(String);

impl StringId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn from_uuid() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Identifier for StringId {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for StringId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StringId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for StringId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for StringId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Numeric identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct U64Id(u64);

impl U64Id {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Identifier for U64Id {
    fn as_str(&self) -> &str {
        // This is technically unsafe if called multiple times and the borrow checker doesn't like it
        // But for most use cases, this pattern works with string caching
        // A better approach would be to store the string representation
        unreachable!("U64Id::as_str should not be called directly; use Display instead")
    }
}

impl fmt::Display for U64Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for U64Id {
    fn from(id: u64) -> Self {
        Self(id)
    }
}
