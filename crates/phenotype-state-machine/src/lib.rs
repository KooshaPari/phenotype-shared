//! # phenotype-state-machine
//!
//! Generic finite state machine with transition guards, validation, and forward-only enforcement.
//!
//! This crate follows **Hexagonal Architecture** (Ports and Adapters) pattern:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    ADAPTERS (Outer)                     │
//! │   HTTP Controllers, CLI, DB, Cache, External Clients    │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ implements
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                   PORTS (Interfaces)                   │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ used by
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                 APPLICATION (Middle)                    │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ uses
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                    DOMAIN (Inner)                       │
//! │    Entities, Services, Ports (no external dependencies)  │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust
//! use phenotype_state_machine::prelude::*;
//! use serde::{Deserialize, Serialize};
//! use std::fmt::{self, Display};
//!
//! // Define your states
//! #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
//! enum OrderState {
//!     Draft, Confirmed, Shipped, Delivered
//! }
//!
//! impl Display for OrderState {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "{:?}", self)
//!     }
//! }
//!
//! impl State for OrderState {
//!     fn ordinal(&self) -> u32 {
//!         match self {
//!             OrderState::Draft => 0,
//!             OrderState::Confirmed => 1,
//!             OrderState::Shipped => 2,
//!             OrderState::Delivered => 3,
//!         }
//!     }
//!     fn all_states() -> Vec<Self> {
//!         vec![OrderState::Draft, OrderState::Confirmed, OrderState::Shipped, OrderState::Delivered]
//!     }
//! }
//!
//! // Use the state machine
//! let mut sm = StateMachine::new(OrderState::Draft);
//! sm.transition(OrderState::Confirmed).unwrap();
//! assert_eq!(*sm.current(), OrderState::Confirmed);
//! ```
//!
//! ## Crate Structure
//!
//! - [`domain`] - Pure domain logic with no external dependencies
//!   - [`domain::entities`] - Core entities: StateMachine, State, TransitionRecord
//!   - [`domain::services`] - Domain services: validators, guards
//!   - [`domain::ports`] - Port interfaces for adapters
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

// Adapters layer (feature-gated)
#[cfg(feature = "persistence-sqlx")]
pub mod adapters_sqlx {
    pub mod persistence_sqlx;
}

#[cfg(feature = "persistence-sqlite")]
pub mod adapters_sqlite {
    pub mod persistence_sqlite;
}

// === PRELUDE ===
// Convenient re-exports for common use
pub mod prelude {
    pub use crate::domain::entities::*;
    pub use crate::domain::services::*;
    pub use crate::domain::ports::*;
    pub use crate::application::dto::*;
}

