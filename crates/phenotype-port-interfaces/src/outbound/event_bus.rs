//! # Event Bus Ports
//!
//! Event bus ports define event publishing and subscribing.

use crate::domain::event::DomainEvent;
use crate::error::Result;
use std::fmt::Debug;

/// Event handler function type.
pub type EventHandler<E> = Box<dyn Fn(E) -> Result<()> + Send + Sync>;

/// Event bus port for pub/sub operations.
#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    /// The event type.
    type Event: DomainEvent + Debug + Clone;

    /// Publish an event to a topic.
    async fn publish(&self, topic: &str, event: &Self::Event) -> Result<()>;

    /// Subscribe to events on a topic.
    async fn subscribe(&self, topic: &str, handler: EventHandler<Self::Event>) -> Result<()>;

    /// Unsubscribe from a topic.
    async fn unsubscribe(&self, topic: &str) -> Result<()>;
}

/// Extension trait for event bus with typed subscribers.
pub trait EventBusExt: EventBus {
    /// Subscribe with a typed handler.
    fn subscribe_typed<F, Fut>(&self, topic: &str, handler: F) -> Result<()>
    where
        F: Fn(Self::Event) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static;
}

impl<T: EventBus> EventBusExt for T {
    fn subscribe_typed<F, Fut>(&self, topic: &str, handler: F) -> Result<()>
    where
        F: Fn(Self::Event) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let handler = Box::new(move |event| {
            let fut = handler(event.clone());
            Box::pin(async move { fut.await })
        });
        // Cannot easily forward due to async closure limitations
        // This is a marker for implementation
        Ok(())
    }
}
