//! # Redis Cache
//!
//! Implementation of the `Cache` port using Redis.

use async_trait::async_trait;
use deadpool_redis::{Config, Pool, Runtime};

use phenotype_port_interfaces::outbound::cache::{Cache, CacheExt};
use phenotype_port_interfaces::error::{PortError, Result as PortResult};

use crate::error::{RedisError, Result};
use crate::redis_config::RedisConfig;

/// Redis implementation of the Cache port.
#[derive(Clone)]
pub struct RedisCache {
    pool: Pool,
}

impl RedisCache {
    /// Create a new RedisCache with the given pool.
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Create a connection pool from config.
    pub async fn from_config(config: &RedisConfig) -> Result<Self> {
        let pool = create_pool(config)?;
        Ok(Self::new(pool))
    }

    fn map_error(e: deadpool_redis::PoolError) -> PortError {
        PortError::ConnectionError(e.to_string())
    }
}

#[async_trait]
impl Cache for RedisCache {
    type Error = PortError;

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        let mut conn = self.pool.get().await.map_err(Self::map_error)?;

        let result: Option<Vec<u8>> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| PortError::StorageError(e.to_string()))?;

        Ok(result)
    }

    async fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        ttl_secs: Option<u64>,
    ) -> Result<(), Self::Error> {
        let mut conn = self.pool.get().await.map_err(Self::map_error)?;

        match ttl_secs {
            Some(ttl) => {
                redis::cmd("SETEX")
                    .arg(key)
                    .arg(ttl as i64)
                    .arg(&value)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| PortError::StorageError(e.to_string()))?;
            }
            None => {
                redis::cmd("SET")
                    .arg(key)
                    .arg(&value)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| PortError::StorageError(e.to_string()))?;
            }
        }

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        let mut conn = self.pool.get().await.map_err(Self::map_error)?;

        redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| PortError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, Self::Error> {
        let mut conn = self.pool.get().await.map_err(Self::map_error)?;

        let exists: bool = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| PortError::StorageError(e.to_string()))?;

        Ok(exists)
    }

    async fn expire(&self, key: &str, ttl_secs: u64) -> Result<(), Self::Error> {
        let mut conn = self.pool.get().await.map_err(Self::map_error)?;

        redis::cmd("EXPIRE")
            .arg(key)
            .arg(ttl_secs as i64)
            .query_async(&mut conn)
            .await
            .map_err(|e| PortError::StorageError(e.to_string()))?;

        Ok(())
    }
}

impl CacheExt for RedisCache {}

#[async_trait]
impl<T: serde::de::DeserializeOwned + Send + Sync> CacheExt for RedisCache {
    async fn get_json<'a, V: serde::Deserialize<'a>>(&self, key: &str) -> Result<Option<V>, PortError> {
        let value = self.get(key).await?;
        match value {
            Some(bytes) => {
                let value: V = serde_json::from_slice(&bytes)
                    .map_err(|e| PortError::InvalidData(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn set_json<T: serde::Serialize>(&self, key: &str, value: &T, ttl_secs: Option<u64>) -> Result<(), PortError> {
        let bytes = serde_json::to_vec(value)
            .map_err(|e| PortError::SerializationError(e))?;
        self.set(key, bytes, ttl_secs).await
    }
}

/// Create a connection pool from config.
pub fn create_pool(config: &RedisConfig) -> Result<Pool> {
    let cfg = Config::from_url(&config.url);

    let mut pool_cfg = deadpool_redis::PoolConfig {
        max_size: config.max_size,
        min_idle: config.min_idle,
        ..Default::default()
    };

    cfg.create_pool(Some(pool_cfg), Runtime::Tokio1)
        .map_err(|e| RedisError::PoolError(e.to_string()))
}
