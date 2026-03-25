//! # phenotype-event-sourcing
//!
//! Generic event sourcing engine for Phenotype.
//!
//! Provides append-only event storage with SHA-256 hash chain verification,
//! snapshot management, and flexible event handling for any serializable event type.
//!
//! This crate follows **Hexagonal Architecture** (Ports and Adapters) pattern:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    ADAPTERS (Outer)                     │
//! │   In-Memory Store, SQLx Store, SQLite Store            │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ implements
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                   PORTS (Interfaces)                   │
//! │   EventRepository, SnapshotRepository                  │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ used by
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                 APPLICATION (Middle)                    │
//! │         Use Cases, DTOs                                │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ uses
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                    DOMAIN (Inner)                       │
//! │    EventEnvelope, Hash Services (no dependencies)       │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust
//! use phenotype_event_sourcing::prelude::*;
//!
//! // Create an event store
//! let store = InMemoryEventStore::new();
//!
//! // Define your event
//! #[derive(Serialize, Deserialize)]
//! struct UserCreated {
//!     user_id: String,
//!     email: String,
//! }
//!
//! // Append an event
//! let event = EventEnvelope::new(
//!     UserCreated {
//!         user_id: "user-123".to_string(),
//!         email: "user@example.com".to_string(),
//!     },
//!     "system",
//! );
//! let sequence = store.append(&event, "User", "user-123").unwrap();
//! ```
//!
//! ## Crate Structure
//!
//! - [`domain`] - Pure domain logic with no external dependencies
//!   - [`domain::entities`] - EventEnvelope
//!   - [`domain::services`] - Hash computation, chain verification
//!   - [`domain::ports`] - Repository interfaces
//! - [`application`] - Use cases and DTOs
//! - [`adapters`] - Infrastructure implementations

// === PUBLIC API ===
// Re-export all public types for ergonomic access

// Domain layer
pub mod domain;
pub use domain::entities;
pub use domain::services;
pub use domain::ports;

// Application layer
pub mod application;
pub use application::dto;
pub use application::use_cases;

// Legacy modules for backward compatibility
pub mod error;
pub mod hash;
pub mod snapshot;

// Adapters layer
pub mod adapters;
pub use adapters::outbound::inmemory::InMemoryEventStore;

// Re-export EventStore trait
pub use domain::ports::outbound::EventRepository;

// Re-export error types
pub use error::{EventSourcingError, EventStoreError, HashError, Result};
pub use hash::{compute_hash, detect_gaps, verify_chain};
pub use snapshot::{should_snapshot, Snapshot, SnapshotConfig};

// === PRELUDE ===
// Convenient re-exports for common use
pub mod prelude {
    pub use crate::domain::entities::*;
    pub use crate::domain::services::*;
    pub use crate::domain::ports::*;
    pub use crate::domain::events::*;
    pub use crate::application::dto::*;
    pub use crate::adapters::outbound::inmemory::InMemoryEventStore;
}

// === TESTS ===
#[cfg(test)]
mod tests {
    mod unit;
}
