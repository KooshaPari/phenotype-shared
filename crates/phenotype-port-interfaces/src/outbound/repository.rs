//! # Repository Ports
//!
//! Repository ports define data persistence capabilities.

use crate::domain::entity::Entity;
use crate::domain::identifier::Identifier;
use crate::error::Result;
use async_trait::async_trait;
use std::fmt::Debug;

/// Marker trait for entities that can be stored in a repository.
pub trait StorableEntity: Entity + Debug + Clone + Send + Sync {
    /// The type of the entity.
    type Id: Identifier;
}

/// Repository port for CRUD operations on entities.
#[async_trait]
pub trait Repository: Send + Sync {
    /// The entity type this repository manages.
    type Entity: StorableEntity;

    /// The identifier type.
    type Id: Identifier;

    /// Find an entity by its ID.
    async fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Entity>>;

    /// Find all entities, with optional pagination.
    async fn find_all(&self, page: u32, page_size: u32) -> Result<Vec<Self::Entity>>;

    /// Save an entity (create or update).
    async fn save(&self, entity: &Self::Entity) -> Result<()>;

    /// Delete an entity by its ID.
    async fn delete(&self, id: &Self::Id) -> Result<()>;

    /// Check if an entity exists.
    async fn exists(&self, id: &Self::Id) -> Result<bool>;
}

/// Extension trait for repositories with common operations.
#[async_trait]
pub trait RepositoryExt: Repository {
    /// Find or create an entity.
    async fn find_or_create<F>(&self, id: &Self::Id, factory: F) -> Result<Self::Entity>
    where
        F: FnOnce() -> Self::Entity + Send,
        Self::Entity: Default + Send,
    {
        Ok(self.find_by_id(id).await?.unwrap_or_else(factory))
    }

    /// Count total entities.
    async fn count(&self) -> Result<u64> {
        let page_size = 1000u32;
        let mut total = 0u64;
        let mut page = 0u32;

        loop {
            let entities = self.find_all(page, page_size).await?;
            let len = entities.len() as u64;
            total += len;

            if len < page_size as u64 {
                break;
            }
            page += 1;
        }

        Ok(total)
    }
}

impl<T: Repository> RepositoryExt for T {}
