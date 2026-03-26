//! # Config Ports
//!
//! Configuration ports define configuration access patterns.

use crate::error::Result;
use serde::de::DeserializeOwned;
use async_trait::async_trait;

/// Configuration port alias for common naming conventions.
pub trait ConfigProvider: Config {}

impl<T: Config> ConfigProvider for T {}

/// Extension trait for config with typed accessors.
#[async_trait]
pub trait Config: Send + Sync {
    /// Get a configuration value by key.
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>>;

    /// Check if a configuration key exists.
    async fn has(&self, key: &str) -> Result<bool>;

    /// Get all keys matching a prefix.
    async fn keys(&self, prefix: &str) -> Result<Vec<String>>;
}

/// Extension trait for config with typed accessors.
#[async_trait]
pub trait ConfigExt: Config {
    /// Get a required configuration value.
    async fn get_required<T: DeserializeOwned + Send>(&self, key: &str) -> Result<T> {
        self.get(key)
            .await?
            .ok_or_else(|| crate::error::PortError::ConfigError(format!("Missing config: {}", key)))
    }

    /// Get with a default value.
    async fn get_or<T: DeserializeOwned + Default + Send>(&self, key: &str) -> Result<T> {
        self.get(key).await.map(|opt| opt.unwrap_or_default())
    }

    /// Get a string with default.
    async fn get_string(&self, key: &str, default: &str) -> Result<String> {
        self.get::<String>(key).await.map(|opt| opt.unwrap_or_else(|| default.to_string()))
    }

    /// Get a boolean with default.
    async fn get_bool(&self, key: &str, default: bool) -> Result<bool> {
        self.get::<bool>(key).await.map(|opt| opt.unwrap_or(default))
    }

    /// Get an integer with default.
    async fn get_u64(&self, key: &str, default: u64) -> Result<u64> {
        self.get::<u64>(key).await.map(|opt| opt.unwrap_or(default))
    }
}

impl<T: Config> ConfigExt for T {}
