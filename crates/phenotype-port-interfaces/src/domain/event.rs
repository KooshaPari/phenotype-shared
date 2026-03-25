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

/// Extension trait for domain events.
/// Extension trait for domain events with helper methods.
pub trait DomainEventExt: DomainEvent {
    /// Creates a new event with auto-generated ID and timestamp.
    fn create(
        correlation_id: Option<String>,
        causation_id: Option<String>,
    ) -> Self
    where
        Self: Sized;
}
    /// Optional correlation ID.
    pub correlation_id: Option<String>,
    /// Optional causation ID.
    pub causation_id: Option<String>,
    /// The actual event payload.
    pub payload: E,
}

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

impl<E: DomainEvent> DomainEventExt for E {
    fn new(
        correlation_id: Option<String>,
        causation_id: Option<String>,
    ) -> Self {
        Self::new(correlation_id, causation_id)
    }
}
