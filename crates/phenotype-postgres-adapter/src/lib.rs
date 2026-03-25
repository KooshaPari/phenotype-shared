//! # Phenotype Postgres Adapter
//!
//! PostgreSQL adapter implementing the `Repository` port for hexagonal architecture.
//!
//! ## Features
//!
//! - **Connection Pooling**: Uses `deadpool-postgres` for efficient connection pooling
//! - **Async/Await**: Fully async implementation using `tokio`
//! - **Repository Pattern**: Implements the `Repository` port interface
//! - **Optimistic Locking**: Built-in version tracking for concurrent updates
//! - **Schema Management**: Auto-creates tables with proper indexes

pub mod error;
pub mod postgres_repository;
pub mod postgres_config;

pub use error::{PostgresError, Result};
pub use postgres_repository::PostgresRepository;
pub use postgres_config::PostgresConfig;

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_exists() {
        // Placeholder test
        assert!(true);
    }
}
