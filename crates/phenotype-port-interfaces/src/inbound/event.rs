//! # Event Ports
//!
//! Event ports define event handling patterns.

use crate::domain::event::DomainEvent;
use crate::error::Result;
use std::fmt::Debug;

/// Event handler port for processing domain events.
#[async_trait::async_trait]
pub trait EventHandler<E: DomainEvent + Debug + Clone>: Send + Sync {
    /// Handle an event.
    async fn handle(&self, event: E) -> Result<()>;
}

/// Event processor for managing multiple event handlers.
#[async_trait::async_trait]
pub trait EventProcessor: Send + Sync {
    /// The event type.
    type Event: DomainEvent + Debug + Clone;

    /// Register a handler for a specific event type.
    fn register<H>(&mut self, handler: H)
    where
        H: EventHandler<Self::Event>;

    /// Process an event through all registered handlers.
    async fn process(&self, event: Self::Event) -> Result<()>;
}
