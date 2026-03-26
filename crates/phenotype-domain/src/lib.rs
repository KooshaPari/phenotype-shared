//!
//! Phenotype Domain Crate - Core domain model following DDD principles.
//!
//! # Architecture
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

#![allow(unused)]
#![allow(missing_docs)]

/// Domain errors: validation and domain rule violations.
/// These represent the canonical ways domain operations can fail.
pub mod errors;

/// Value objects: immutable, self-validating domain primitives.
/// Once created, value objects cannot be changed - they are replaced, not mutated.
pub mod value_objects;

/// Entities: domain objects with identity that persists over time.
/// Entities have lifecycle state and their identity is stable across state changes.
pub mod entities;

/// Aggregates: clusters of related entities treated as a single unit.
/// Aggregates own a root entity and enforce invariants for the entire cluster.
pub mod aggregates;

/// Domain events: immutable facts.
/// Events represent something that happened in the domain and can never be undone.
/// They are the basis for event sourcing and auditing.
pub mod events;

/// Domain services: stateless cross-boundary operations.
/// Services encapsulate operations that don't naturally belong to a single entity.
pub mod services;

// ------------------------------------------------------------------------------------------------
// Re-exports for convenience
// ------------------------------------------------------------------------------------------------

pub use errors::{DomainError, ValidationError};
pub use value_objects::*;
