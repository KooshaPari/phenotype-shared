//! # HTTP Configuration

use serde::{Deserialize, Serialize};

/// Configuration for HTTP client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub timeout_secs: u64,
    pub user_agent: Option<String>,
    pub default_headers: std::collections::HashMap<String, String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            user_agent: Some("phenotype-http-adapter/0.1.0".to_string()),
            default_headers: std::collections::HashMap::new(),
        }
    }
}

impl HttpConfig {
    /// Create a new config with timeout.
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            timeout_secs,
            ..Default::default()
        }
    }

    /// Add a default header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    /// Set user agent.
    pub fn with_user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }
}
