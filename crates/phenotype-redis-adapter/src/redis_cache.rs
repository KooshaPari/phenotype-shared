//! # Redis Cache
//!
//! Simple Redis cache implementation.

use deadpool_redis::redis::RedisError;
use deadpool_redis::{Config, Pool, Runtime};

use crate::error::RedisError as AppRedisError;
use crate::redis_config::RedisConfig;

/// Redis cache for storing key-value pairs.
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
    pub fn from_config(config: &RedisConfig) -> Result<Self, AppRedisError> {
        let pool = create_pool(config)?;
        Ok(Self::new(pool))
    }

    /// Get a value by key.
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, AppRedisError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppRedisError::Pool(e.to_string()))?;
        let result: Result<Option<Vec<u8>>, RedisError> = deadpool_redis::redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await;
        result.map_err(|e| AppRedisError::Query(e.to_string()))
    }

    /// Set a value with optional TTL.
    pub async fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        ttl_secs: Option<u64>,
    ) -> Result<(), AppRedisError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppRedisError::Pool(e.to_string()))?;
        match ttl_secs {
            Some(ttl) => {
                let result: Result<(), RedisError> = deadpool_redis::redis::cmd("SETEX")
                    .arg(key)
                    .arg(ttl as i64)
                    .arg(&value)
                    .query_async(&mut conn)
                    .await;
                result.map_err(|e| AppRedisError::Query(e.to_string()))?;
            }
            None => {
                let result: Result<(), RedisError> = deadpool_redis::redis::cmd("SET")
                    .arg(key)
                    .arg(&value)
                    .query_async(&mut conn)
                    .await;
                result.map_err(|e| AppRedisError::Query(e.to_string()))?;
            }
        }
        Ok(())
    }

    /// Delete a key.
    pub async fn delete(&self, key: &str) -> Result<(), AppRedisError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppRedisError::Pool(e.to_string()))?;
        let result: Result<i64, RedisError> = deadpool_redis::redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await;
        result.map_err(|e| AppRedisError::Query(e.to_string()))?;
        Ok(())
    }

    /// Check if a key exists.
    pub async fn exists(&self, key: &str) -> Result<bool, AppRedisError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppRedisError::Pool(e.to_string()))?;
        let result: Result<i64, RedisError> = deadpool_redis::redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut conn)
            .await;
        let count = result.map_err(|e| AppRedisError::Query(e.to_string()))?;
        Ok(count > 0)
    }
}

/// Create a connection pool from config.
pub fn create_pool(config: &RedisConfig) -> Result<Pool, AppRedisError> {
    let cfg = Config::from_url(&config.url);
    cfg.create_pool(Some(Runtime::Tokio1))
        .map_err(|e| AppRedisError::Pool(e.to_string()))
}
