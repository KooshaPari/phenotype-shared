//! # Prelude Module
//!
//! Convenient re-exports for common use cases.

pub use crate::domain::{
    Entity, ValueObject, ValueObjectExt,
    DomainEvent, EventMetadata,
    AggregateRoot, Aggregate,
    Identifier, StringId, U64Id, UuidIdentifier,
};
pub use crate::outbound::{
    Repository, RepositoryExt,
    Cache, CacheExt,
    Queue, EventQueue, Message,
    EventBus, EventHandler,
    ConfigProvider,
    Logger,
    HttpClient, HttpRequest, HttpResponse, HttpMethod,
    FileSystem, FileSystemExt,
};
pub use crate::inbound::{
    Command, CommandHandler, CommandBus, CommandBusExt,
    Query, QueryHandler, QueryBus,
    Paginated,
};
pub use crate::error::{PortError, Result};
