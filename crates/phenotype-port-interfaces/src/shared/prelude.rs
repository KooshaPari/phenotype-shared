//! # Prelude Module
//!
//! Convenient re-exports for common use cases.

pub use crate::domain::{
    Aggregate, DomainEvent, Entity, EventMetadata, Identifier, StringId, UuidIdentifier,
    ValueObject, ValueObjectExt,
};
pub use crate::error::{PortError, Result};
pub use crate::inbound::{Command, CommandBus, CommandHandler, Query, QueryBus, QueryHandler};
pub use crate::outbound::{
    Cache, CacheExt, ConfigProvider, EventBus, EventHandler, EventQueue, FileSystem, HttpClient,
    HttpRequest, HttpResponse, Logger, Message, Queue, Repository, RepositoryExt,
};
