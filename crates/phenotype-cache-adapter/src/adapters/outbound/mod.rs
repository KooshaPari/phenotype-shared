//! Outbound adapters - Infrastructure implementations.
//!
//! These adapters implement the outbound ports defined in the domain:
//! - EntryStore: In-memory tier implementations
//! - MetricsCollector: Metrics collection implementations

pub mod memory;
pub mod metrics;

use memory::in_memory::InMemoryEntryStore;
use metrics::default::DefaultMetricsCollector;

pub use memory::in_memory::InMemoryTier;
pub use metrics::default::NoopMetricsCollector;
