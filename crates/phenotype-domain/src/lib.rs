//! # Phenotype Domain Core
//!
//! The domain core is the heart of the Phenotype system, implementing
//! **Domain-Driven Design** principles with a clean, hexagonal architecture.
//!
//! ## Zero Dependencies
//!
//! This crate has **zero external runtime dependencies**. This is a deliberate
//! design choice enabling:
//!
//! - Embedding in any runtime (WASM, embedded, CLI, server)
//! - No dependency conflicts across adapters
//! - Fast compile times
//! - Clear separation of concerns
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    DOMAIN CORE                          │
//! │                  (phenotype-domain)                     │
//! │                                                         │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │
//! │  │   Value     │  │  Entities   │  │   Aggregates    │ │
//! │  │  Objects    │  │             │  │   (Roots)       │ │
//! │  └─────────────┘  └─────────────┘  └─────────────────┘ │
//! │                                                         │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │
//! │  │   Domain    │  │   Domain    │  │     Error       │ │
//! │  │   Events    │  │  Services   │  │    Types        │ │
//! │  └─────────────┘  └─────────────┘  └─────────────────┘ │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## DDD Building Blocks
//!
//! | Component | Description | Example |
//! |-----------|-------------|---------|  
//! | **Value Objects** | Immutable, equality by value | `AgentId`, `TaskStatus` |
//! | **Entities** | Identity-based, mutable | `Agent`, `Task` |
//! | **Aggregates** | Consistency boundary, root | `AgentAggregate` |
//! | **Domain Events** | Immutable facts | `AgentCreated` |
//! | **Domain Services** | Stateless operations | `AgentFactory` |
//! | **Repositories** | Persistence abstraction (port) | `AgentRepository` |
//!
//! ## Bounded Contexts
//!
//! - **Agent Context** - Agent lifecycle, capabilities, health
//! - **Task Context** - Task management, assignment, execution
//! - **Workflow Context** - Multi-step workflow orchestration
//! - **Policy Context** - Governance, security, compliance

// === Public API ===

pub mod errors;
pub mod value_objects;
pub mod entities;
pub mod aggregates;
pub mod events;
pub mod services;

pub use errors::{DomainError, DomainResult, ValidationError};
pub use value_objects::*;
pub use entities::*;
pub use aggregates::*;
pub use events::*;
pub use services::*;
