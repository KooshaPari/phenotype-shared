//! # Domain Layer
//!
//! Core domain concepts without any external dependencies.
//!
//! These types represent the universal language of the Phenotype ecosystem
//! and should have no knowledge of how they're persisted or transmitted.

pub mod entity;
pub mod value_object;
pub mod event;
pub mod aggregate;
pub mod identifier;

pub use entity::*;
pub use value_object::*;
pub use event::*;
pub use aggregate::*;

// Re-export commonly used aliases
pub use aggregate::Aggregate as AggregateRoot;
pub use identifier::{Identifier, StringId, U64Id, UuidIdentifier};
