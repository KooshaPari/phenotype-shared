//! # Domain Layer
//!
//! Core domain concepts without any external dependencies.
//!
//! These types represent the universal language of the Phenotype ecosystem
//! and should have no knowledge of how they're persisted or transmitted.

pub mod aggregate;
pub mod entity;
pub mod event;
pub mod identifier;
pub mod value_object;

pub use aggregate::*;
pub use entity::*;
pub use event::*;
pub use value_object::*;

// Re-export commonly used identifiers
pub use identifier::{Identifier, StringId, U64Id, UuidIdentifier};
