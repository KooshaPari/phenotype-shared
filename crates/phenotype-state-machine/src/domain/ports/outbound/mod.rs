//! # Outbound Ports (Secondary/Driven Ports)
//!
//! These ports define the **capabilities** that infrastructure must provide.
//! The domain defines these traits; adapters implement them.
//!
//! ## Common Outbound Ports
//!
//! | Port | Purpose |
//! |------|---------|
//! | `StateMachineRepository` | Persist and retrieve state machines |
//! | `EventPublisher` | Publish state transition events |
//!
//! ## Example
//!
//! ```rust,ignore
//! use crate::domain::ports::outbound::StateMachineRepository;
//! use crate::domain::entities::StateMachine;
//! use crate::domain::value_objects::StateMachineId;
//!
//! // Outbound port trait (defined by domain)
//! pub trait StateMachineRepository: Send + Sync {
//!     fn save(&self, sm: &StateMachine<S>) -> Result<(), RepositoryError>;
//!     fn find_by_id(&self, id: &StateMachineId) -> Result<Option<StateMachine<S>>, RepositoryError>;
//! }
//!
//! // Adapter implements the port (in adapters crate)
//! pub struct SqlxStateMachineRepository { pool: PgPool }
//!
//! impl StateMachineRepository for SqlxStateMachineRepository { ... }
//! ```
//!
//! ## Dependency Rule
//!
//! ```text
//! Domain defines port ──implemented by──► Adapter
//! ```
use super::super::entities::{StateMachine, TransitionRecord};
use std::fmt::Debug;

/// Errors that can occur in repositories.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("persist error: {0}")]
    Persist(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}

/// Port for persisting state machines.
pub trait StateMachineRepository<S: super::entities::State>: Send + Sync {
    /// Save a state machine.
    fn save(&self, id: &str, sm: &StateMachine<S>) -> Result<(), RepositoryError>;

    /// Find a state machine by ID.
    fn find_by_id(&self, id: &str) -> Result<Option<StateMachine<S>>, RepositoryError>;

    /// Delete a state machine by ID.
    fn delete(&self, id: &str) -> Result<(), RepositoryError>;
}

/// Port for publishing state transition events.
pub trait TransitionEventPublisher: Send + Sync {
    /// Publish a transition event.
    fn publish(&self, event: TransitionEvent) -> Result<(), PublisherError>;
}

/// Errors that can occur in event publishers.
#[derive(Debug, thiserror::Error)]
pub enum PublisherError {
    #[error("publish error: {0}")]
    Publish(String),

    #[error("connection error: {0}")]
    Connection(String),
}

/// Event published when a state transition occurs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransitionEvent {
    pub id: String,
    pub from_state: String,
    pub to_state: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
