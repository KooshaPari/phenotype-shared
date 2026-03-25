//! # Phenotype Redis Adapter
//!
//! Redis adapter implementing the `Cache` port for hexagonal architecture.
//!
//! ## Features
//!
//! - **Connection Pooling**: Uses `deadpool-redis` for efficient connection pooling
//! - **Async/Await**: Fully async implementation using `tokio`
//! - **TTL Support**: Built-in expiration support
//! - **Serialization**: JSON serialization for cached values

pub mod error;
pub mod redis_cache;
pub mod redis_config;

pub use error::{RedisError, Result};
pub use redis_cache::RedisCache;
pub use redis_config::RedisConfig;

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_exists() {
        assert!(true);
    }
}
