//! # Cache Ports
//!
//! Cache ports define caching capabilities.

use crate::error::{PortError, Result};
use std::time::Duration;

/// Cache port for key-value caching operations.
#[async_trait::async_trait]
pub trait Cache: Send + Sync {
    /// The key type.
    type Key: AsRef<str> + Send + Sync;
    /// The value type.
    type Value: Send + Sync;

    /// Get a value from the cache.
    async fn get(&self, key: &Self::Key) -> Result<Option<Self::Value>>;

    /// Set a value in the cache with optional TTL.
    async fn set(&self, key: &Self::Key, value: &Self::Value, ttl: Option<Duration>) -> Result<()>;

    /// Delete a key from the cache.
    async fn delete(&self, key: &Self::Key) -> Result<()>;

    /// Check if a key exists.
    async fn exists(&self, key: &Self::Key) -> Result<bool>;

    /// Clear all entries from the cache.
    async fn clear(&self) -> Result<()>;
}

/// Extension trait for cache with convenience methods.
pub trait CacheExt: Cache {
    /// Get a value or compute and cache it.
    async fn get_or_compute<F, Fut>(&self, key: &Self::Key, factory: F) -> Result<Self::Value>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Self::Value>>,
    {
        if let Some(value) = self.get(key).await? {
            return Ok(value);
        }

        let value = factory().await?;
        self.set(key, &value, None).await?;
        Ok(value)
    }

    /// Get with TTL.
    async fn get_ttl(&self, key: &Self::Key, ttl: Duration) -> Result<Option<Self::Value>> {
        let value = self.get(key).await?;
        if value.is_some() {
            self.set(key, value.as_ref().unwrap(), Some(ttl)).await?;
        }
        Ok(value)
    }
}

impl<T: Cache> CacheExt for T {}
