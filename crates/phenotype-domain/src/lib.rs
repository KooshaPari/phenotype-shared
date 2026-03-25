//! # Phenotype Domain
//!
//! Core domain model following DDD (Domain-Driven Design) principles.
//!
//! ## Architecture
//!
//! This crate follows the xDD methodology stack:
//! - **DDD**: Bounded contexts, aggregates, entities, value objects
//! - **TDD**: Tests written before implementation
//! - **BDD**: Behavior-driven design with clear specifications
//! - **Clean Architecture**: Domain core with no external dependencies
//! - **Hexagonal Architecture**: Ports and adapters isolation
//!
//! ## Bounded Contexts
//!
//! - **Agent Context**: Agent lifecycle, capabilities, routing
//! - **Task Context**: Task creation, execution, completion
//! - **Workflow Context**: Multi-step workflow orchestration
//! - **Policy Context**: Governance, security, compliance
//!
//! ## Building Blocks
//!
//! - **Value Objects**: Immutable, interchangeable by identity
//! - **Entities**: Has identity, mutable state
//! - **Aggregates**: Consistency boundary, root entity
//! - **Domain Events**: Immutable facts that happened
//! - **Domain Services**: Stateless operations crossing aggregate boundaries

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_crates_documentation)]

/// Validation and domain rule errors.
pub mod errors;

/// Value objects: immutable, interchangeable by identity.
pub mod value_objects;

/// Entities: have identity, mutable state.
pub mod entities;

/// Aggregates: consistency boundary, root entity.
pub mod aggregates;

/// Domain events: immutable facts.
pub mod events;

/// Domain services: stateless cross-boundary operations.
pub mod services;

// ------------------------------------------------------------------------------------------------
// Re-exports for convenience
// ------------------------------------------------------------------------------------------------

pub use errors::{DomainError, ValidationError};
pub use value_objects::*;
pub use entities::*;
pub use aggregates::*;
pub use events::*;
