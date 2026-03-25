//! # Aggregate
//!
//! Aggregates are clusters of related entities and value objects
//! that are treated as a single unit for data changes.
//!
//! See [Domain-Driven Design](https://martinfowler.com/bliki/DDD_Aggregate.html)

use super::event::DomainEvent;

/// Marker trait for aggregates.
///
/// Aggregates are the primary unit of persistence and consistency.
/// They ensure invariants are maintained within their boundary.
pub trait Aggregate: 'static + Send + Sync {
    /// The type of identifier for this aggregate.
    type Id;

    /// The type of uncommitted events produced by this aggregate.
    type Event: DomainEvent;

    /// Returns the aggregate ID.
    fn id(&self) -> &Self::Id;

    /// Returns the current version of the aggregate (0 if new).
    fn version(&self) -> i64;

    /// Takes pending events and clears the queue.
    fn take_events(&mut self) -> Vec<Self::Event>;

    /// Applies an event to the aggregate.
    fn apply(&mut self, event: Self::Event);
}

/// Extension trait for aggregates with helper methods.
pub trait AggregateExt: Aggregate {
    /// Returns true if there are pending events.
    fn has_pending_events(&self) -> bool {
        self.version() == 0 // Simplified check
    }
}

impl<T: Aggregate> AggregateExt for T {}
