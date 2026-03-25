//! # Phenotype Port Interfaces
//!
//! Shared port interfaces for **Hexagonal Architecture** across the Phenotype ecosystem.
//!
//! This crate provides **domain-agnostic, technology-agnostic** port interfaces that can be
//! implemented by any adapter (database, cache, HTTP client, file system, etc.).
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │                    APPLICATION LAYER                         │
//! │              (Use Cases, Commands, Queries)                  │
//! └─────────────────────────┬──────────────────────────────────┘
//!                            │ depends on
//!                            ▼
//! ┌──────────────────────────────────────────────────────────────┐
//! │               PHENOTYPE-PORT-INTERFACES                      │
//! │                    (This Crate)                              │
//! │  ┌─────────────────────────────────────────────────────────┐ │
//! │  │                 OUTBOUND PORTS                            │ │
//! │  │  Repository, Cache, Queue, EventBus, Logger, Config      │ │
//! │  └─────────────────────────────────────────────────────────┘ │
//! │  ┌─────────────────────────────────────────────────────────┐ │
//! │  │                 INBOUND PORTS                            │ │
//! │  │  CommandHandler, QueryHandler, EventHandler              │ │
//! │  └─────────────────────────────────────────────────────────┘ │
//! └─────────────────────────┬──────────────────────────────────┘
//!                            │ implemented by
//!                            ▼
//! ┌──────────────────────────────────────────────────────────────┐
//! │                     ADAPTERS LAYER                           │
//! │       Postgres, Redis, Kafka, HTTP, File System              │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use phenotype_port_interfaces::outbound::repository::{Repository, Entity};
//!
//! // Define your entity
//! #[derive(Entity)]
//! struct User {
//!     id: String,
//!     email: String,
//! }
//!
//! // Implement the repository port
//! impl Repository for User {
//!     type Id = String;
//!     fn id(&self) -> &Self::Id { &self.id }
//! }
//! ```
//!
//! ## Port Categories
//!
//! ### Outbound Ports (Driven/Secondary)
//! - [`outbound::repository`] - Data persistence
//! - [`outbound::cache`] - Caching operations
//! - [`outbound::queue`] - Message queue operations
//! - [`outbound::event_bus`] - Event publishing/subscribing
//! - [`outbound::config`] - Configuration access
//! - [`outbound::logger`] - Logging operations
//! - [`outbound::http`] - HTTP client operations
//!
//! ### Inbound Ports (Driving/Primary)
//! - [`inbound::command`] - Command handlers
//! - [`inbound::query`] - Query handlers
//! - [`inbound::event`] - Event handlers

// === PUBLIC API ===

/// Domain layer - core concepts without external dependencies
pub mod domain;
pub use domain::*;

/// Outbound (secondary/driven) port interfaces - what infrastructure must provide
pub mod outbound;
pub use outbound::*;

/// Inbound (primary/driving) port interfaces - what application exposes
pub mod inbound;
pub use inbound::*;

/// Cross-cutting concerns
pub mod shared;
pub use shared::*;

// === ERROR TYPES ===

/// Common error types used across ports
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum PortError {
        #[error("Not found: {0}")]
        NotFound(String),

        #[error("Already exists: {0}")]
        AlreadyExists(String),

        #[error("Validation error: {0}")]
        ValidationError(String),

        #[error("Storage error: {0}")]
        StorageError(String),

        #[error("Connection error: {0}")]
        ConnectionError(String),

        #[error("Timeout error: {0}")]
        Timeout(String),

        #[error("Permission denied: {0}")]
        PermissionDenied(String),

        #[error("Invalid state: {0}")]
        InvalidState(String),

        #[error("Serialization error: {0}")]
        SerializationError(#[from] serde_json::Error),

        #[error("Configuration error: {0}")]
        ConfigError(String),
    }

    /// Result type alias for port operations
    pub type Result<T> = std::result::Result<T, PortError>;
}

// === TESTS ===
#[cfg(test)]
mod tests {
    mod unit;
}
