//!
//! # Phenotype Application Layer
//!
//! This crate implements the **CQRS (Command Query Responsibility Segregation)** pattern
//! from the xDD methodology stack.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                        APPLICATION LAYER                              │
////! │                                                                      │
//! │  ┌─────────────────────┐     ┌─────────────────────┐               │
//! │  │      COMMANDS       │     │       QUERIES        │               │
//! │  │  (Write Operations) │     │   (Read Operations)  │               │
//! │  │                     │     │                     │               │
//! │  │  - CreateAgent      │     │  - GetAgentById     │               │
//! │  │  - UpdateAgent      │     │  - ListAgents       │               │
//! │  │  - DeleteAgent      │     │  - SearchAgents     │               │
//! │  │  - CreateTask       │     │  - GetTaskMetrics   │               │
//! │  │  - AssignTask       │     │  - ListTasksByAgent │               │
//! │  └──────────┬──────────┘     └──────────┬──────────┘               │
//! │             │                            │                           │
//! │             ▼                            ▼                           │
//! │  ┌─────────────────────────────────────────────────────────┐       │
//! │  │                    USE CASE HANDLERS                      │       │
//! │  │  - Validate command/query                                 │       │
//! │  │  - Coordinate domain objects                             │       │
//! │  │  - Orchestrate ports (inbound)                          │       │
//! │  │  - Return DTOs                                          │       │
//! │  └─────────────────────────────────────────────────────────┘       │
//! │                              │                                     │
//! └──────────────────────────────┼─────────────────────────────────────┘
//!                                │
//!                                ▼
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                          DOMAIN LAYER                               │
//! │  phenotype-domain: entities, aggregates, value objects, events      │
//! └─────────────────────────────────────────────────────────────────────┘
//!                                │
//!                                ▼
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                       PORTS (INTERFACES)                            │
//! │  phenotype-port-interfaces: inbound and outbound port traits        │
//! └─────────────────────────────────────────────────────────────────────┘
//!                                │
//!                                ▼
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                    INFRASTRUCTURE ADAPTERS                          │
//! │  phenotype-postgres-adapter, phenotype-redis-adapter, etc.         │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Design Principles (xDD)
//!
//! - **SpecDD**: Every command/query has a spec in doc comments
//! - **TDD**: Handlers tested with mock ports
//! - **BDD**: Command/query names follow "Verb+Noun" pattern
//! - **Clean Architecture**: No infrastructure deps in application layer
//! - **Hexagonal**: Application layer is the "primary adapter"
//!
//! ## Commands vs Queries (CQRS)
//!
//! | Aspect | Commands | Queries |
//! |--------|---------|--------|
//! | Purpose | Modify state | Read state |
//! | Return | Result with events | Result with DTO |
//! | Validation | Full domain validation | Minimal validation |
//! | Caching | Don't cache | Cache aggressively |
//! | Concurrency | Optimistic locking | No locks needed |

#![allow(unused)]
#![allow(missing_docs)]

pub mod commands;
pub mod queries;
pub mod handlers;
pub mod dto;

// Re-exports
pub use commands::*;
pub use queries::*;
pub use handlers::*;
pub use dto::*;

/// Application layer error type.
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),

    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl From<phenotype_domain::DomainError> for ApplicationError {
    fn from(e: phenotype_domain::DomainError) -> Self {
        ApplicationError::DomainError(e.to_string())
    }
}

impl From<phenotype_port_interfaces::error::PortError> for ApplicationError {
    fn from(e: phenotype_port_interfaces::error::PortError) -> Self {
        ApplicationError::InfrastructureError(e.to_string())
    }
}
