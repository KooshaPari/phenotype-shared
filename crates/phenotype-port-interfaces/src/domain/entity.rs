//! # Entity Trait
//!
//! Base trait for domain entities with identity.

use chrono::{DateTime, Utc};

use super::identifier::Identifier;

/// Marker trait for entities that have a stable identity.
///
/// Entities are distinguished by their identity, not by their attributes.
/// Two entities with the same ID are the same entity.
pub trait Entity: 'static + Send + Sync {
    /// The type of identifier for this entity.
    type Id: Identifier;

    /// Returns the unique identifier of this entity.
    fn id(&self) -> &Self::Id;

    /// Returns when this entity was created.
    fn created_at(&self) -> DateTime<Utc>;

    /// Returns when this entity was last modified.
    fn updated_at(&self) -> DateTime<Utc>;
}

/// Extension methods for entities.
pub trait EntityExt: Entity {
    /// Check if this entity is the same as another by ID comparison.
    fn is_same(&self, other: &impl Entity<Id = Self::Id>) -> bool {
        self.id() == other.id()
    }
}

impl<T: Entity> EntityExt for T {}
