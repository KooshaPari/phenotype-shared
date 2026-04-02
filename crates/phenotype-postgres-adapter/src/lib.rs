//! # Phenotype Postgres Adapter
//!
//! PostgreSQL adapter for hexagonal architecture.

pub mod error;
pub mod postgres_config;
pub mod postgres_repository;

pub use error::PostgresError;
pub use postgres_config::PostgresConfig;
pub use postgres_repository::PostgresRepository;

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_exists() {
        assert!(true);
    }
}
