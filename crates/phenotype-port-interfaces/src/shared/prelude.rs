//! # Prelude Module
//!
//! Convenient re-exports for common use cases.

pub use crate::domain::{
    Entity,
    ValueObject,
    ValueObjectExt,
    DomainEvent,
    EventMetadata,
    Aggregate,
    Identifier,
    StringId,
    UuidIdentifier,
};
pub use crate::outbound::{
    Repository,
    RepositoryExt,
    Cache,
    CacheExt,
    Queue,
    EventQueue,
    Message,
    EventBus,
    EventHandler,
    ConfigProvider,
    Logger,
    HttpClient,
    HttpRequest,
    HttpResponse,
    FileSystem,
};
pub use crate::inbound::{
    Command,
    CommandHandler,
    CommandBus,
    Query,
    QueryHandler,
    QueryBus,
};
pub use crate::error::{PortError, Result};
