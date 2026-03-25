//! # Prelude Module
//!
//! Convenient re-exports for common use cases.

pub use crate::domain::{
    Entity, EntityExt, ValueObject, ValueObjectExt,
    DomainEvent, DomainEventExt, EventEnvelope,
    Aggregate, AggregateExt,
    Identifier, StringId, U64Id,
};
pub use crate::outbound::{
    Repository, RepositoryExt, StorableEntity,
    Cache, CacheExt,
    Queue, EventQueue, Message,
    EventBus, EventBusExt,
    Config, ConfigExt,
    Logger, LoggerExt, LogLevel, LogRecord,
    HttpClient, HttpMethod, HttpRequest, HttpResponse,
    FileSystem, FileSystemExt,
};
pub use crate::inbound::{
    Command, CommandHandler, CommandBus, CommandBusExt,
    Query, QueryHandler, QueryBus,
    EventHandler, EventProcessor,
    Paginated,
};
pub use crate::error::{PortError, Result};
