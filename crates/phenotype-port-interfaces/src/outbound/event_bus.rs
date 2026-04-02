//! # Event Bus Ports
//!
//! Event bus ports define event publishing and subscribing.

use crate::domain::event::DomainEvent;
use crate::error::Result;
use async_trait::async_trait;
use std::fmt::Debug;

/// Event handler function type.
pub type EventHandler<E> = Box<dyn Fn(E) -> Result<()> + Send + Sync>;

/// Event bus port for pub/sub operations.
#[async_trait]
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

/// Extension trait for EventBus with additional helpers.
#[async_trait]
pub trait EventBusExt: EventBus {
    /// Publish multiple events to a topic.
    async fn publish_all(&self, topic: &str, events: &[Self::Event]) -> Result<()> {
        for event in events {
            self.publish(topic, event).await?;
        }
        Ok(())
    }
}

impl<T: EventBus> EventBusExt for T {}
