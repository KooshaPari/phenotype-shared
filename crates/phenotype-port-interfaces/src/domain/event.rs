//! # Domain Events
//!
//! Domain events represent significant occurrences in the business domain.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A domain event representing something that happened in the domain.
pub trait DomainEvent: 'static + Send + Sync + Clone + serde::Serialize {
    /// The unique type name of this event.
    fn event_type(&self) -> &'static str;

    /// The unique ID of this event instance.
    fn event_id(&self) -> &str;

    /// When this event occurred.
    fn occurred_at(&self) -> DateTime<Utc>;

    /// Optional correlation ID for tracing.
    fn correlation_id(&self) -> Option<&str>;

    /// Optional causation ID for tracking event chains.
    fn causation_id(&self) -> Option<&str>;
}

/// Base event envelope containing metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<E> {
    /// Unique event ID.
    pub id: String,
    /// Event type name.
    pub event_type: &'static str,
    /// When the event occurred.
    pub occurred_at: DateTime<Utc>,
    /// Optional correlation ID.
    pub correlation_id: Option<String>,
    /// Optional causation ID.
    pub causation_id: Option<String>,
    /// The actual event payload.
    pub payload: E,
}

/// Event metadata containing tracing and context information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Unique event ID.
    pub id: String,
    /// When the event occurred.
    pub occurred_at: DateTime<Utc>,
    /// Optional correlation ID for tracing.
    pub correlation_id: Option<String>,
    /// Optional causation ID for tracking event chains.
    pub causation_id: Option<String>,
}

impl EventMetadata {
    /// Creates new event metadata with a generated ID and timestamp.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            occurred_at: Utc::now(),
            correlation_id: None,
            causation_id: None,
        }
    }

    /// Sets the correlation ID.
    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Sets the causation ID.
    pub fn with_causation_id(mut self, causation_id: String) -> Self {
        self.causation_id = Some(causation_id);
        self
    }
}

impl Default for EventMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Event envelope impl
impl<E> EventEnvelope<E> {
    /// Creates a new event envelope.
    pub fn new(
        payload: E,
        event_type: &'static str,
        correlation_id: Option<String>,
        causation_id: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type,
            occurred_at: Utc::now(),
            correlation_id,
            causation_id,
            payload,
        }
    }

    /// Maps the payload to a different type.
    pub fn map<F, U>(self, f: F) -> EventEnvelope<U>
    where
        F: FnOnce(E) -> U,
    {
        EventEnvelope {
            id: self.id,
            event_type: self.event_type,
            occurred_at: self.occurred_at,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
            payload: f(self.payload),
        }
    }
}
