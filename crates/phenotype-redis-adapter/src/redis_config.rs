//! # Redis Configuration

use serde::{Deserialize, Serialize};

/// Configuration for Redis connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_size: usize,
    pub min_idle: Option<usize>,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            max_size: 16,
            min_idle: Some(4),
        }
    }
}

impl RedisConfig {
    /// Create a new config with URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            max_size: 16,
            min_idle: Some(4),
        }
    }

    /// Create a new config with full options.
    pub fn with_options(url: impl Into<String>, max_size: usize, min_idle: Option<usize>) -> Self {
        Self {
            url: url.into(),
            max_size,
            min_idle,
        }
    }
}
