//! # Queue Ports
//!
//! Queue ports define message queue operations.

use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Message envelope for queue operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T = serde_json::Value> {
    /// Unique message ID.
    pub id: String,
    /// Message payload.
    pub payload: T,
    /// Optional correlation ID.
    pub correlation_id: Option<String>,
    /// Optional headers.
    pub headers: std::collections::HashMap<String, String>,
    /// When the message was enqueued.
    pub enqueued_at: chrono::DateTime<chrono::Utc>,
    /// Optional delay before delivery.
    pub delay: Option<std::time::Duration>,
}

impl<T> Message<T> {
    /// Create a new message.
    pub fn new(payload: T) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            payload,
            correlation_id: None,
            headers: std::collections::HashMap::new(),
            enqueued_at: chrono::Utc::now(),
            delay: None,
        }
    }

    /// Add a correlation ID.
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }

    /// Add a header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set a delay.
    pub fn with_delay(mut self, delay: std::time::Duration) -> Self {
        self.delay = Some(delay);
        self
    }
}

/// Queue port for message queue operations.
#[async_trait]
pub trait Queue: Send + Sync {
    /// The message type.
    type Message: Send;

    /// Enqueue a message.
    async fn enqueue(&self, message: Self::Message) -> Result<()>;

    /// Dequeue a message (blocking with timeout).
    async fn dequeue(&self, timeout: std::time::Duration) -> Result<Option<Self::Message>>;

    /// Acknowledge successful processing.
    async fn ack(&self, message_id: &str) -> Result<()>;

    /// Negative acknowledge (retry or move to DLQ).
    async fn nack(&self, message_id: &str) -> Result<()>;
}

/// Event queue port for event sourcing.
#[async_trait]
pub trait EventQueue<E: Clone + Send + Sync + serde::Serialize>: Send + Sync {
    /// Publish an event.
    async fn publish(
        &self,
        topic: &str,
        event: &crate::domain::event::EventEnvelope<E>,
    ) -> Result<()>;

    /// Subscribe to events on a topic.
    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn FnMut(crate::domain::event::EventEnvelope<E>) -> Result<()> + Send>,
    ) -> Result<()>;
}
