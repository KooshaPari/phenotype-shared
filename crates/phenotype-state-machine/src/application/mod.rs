//! # Application Layer
//!
//! Orchestrates domain logic and coordinates between inbound and outbound ports.
//!
//! This layer contains:
//! - [`use_cases`] - Application use cases
//! - [`dto`] - Data Transfer Objects
//!
//! ## Dependency Rule
//!
//! ```text
//! Application ──depends on──► Domain
//! Application ──uses ports──► Outbound Ports
//! ```

pub mod use_cases;
pub mod dto;

// Re-exports
pub use use_cases::*;
pub use dto::*;
