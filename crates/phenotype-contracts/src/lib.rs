//! Core contracts and abstractions for the Phenotype ecosystem.
//!
//! This crate defines the fundamental interfaces and types that other
//! Phenotype crates depend on, ensuring loose coupling and clear contracts.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Re-export common serde types for convenience.
pub mod serde {
    pub use serde::{Deserialize, Serialize};
}

/// Standard result type used across phenotype crates.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Core contract trait for domain events and messages.
///
/// This trait defines the basic contract interface used across the phenotype
/// ecosystem for event sourcing, messaging, and domain event handling.
pub trait Contract: Send + Sync {
    /// Returns the type of this contract.
    fn contract_type(&self) -> &'static str;

    /// Returns the timestamp when this contract was created.
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;

    /// Returns the correlation ID for tracking related events.
    fn correlation_id(&self) -> uuid::Uuid;

    /// Returns a reference to this object as Any for downcasting.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Event trait for domain events.
///
/// This trait extends Contract and adds event-specific properties like
/// aggregate ID and sequence number for event sourcing.
pub trait Event: Contract {
    /// Returns the aggregate ID this event belongs to.
    fn aggregate_id(&self) -> &str;

    /// Returns the sequence number of this event in the aggregate's stream.
    fn sequence(&self) -> u64;
}
