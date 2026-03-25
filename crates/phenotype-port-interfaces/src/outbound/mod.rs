//! # Outbound Ports
//!
//! Outbound (secondary/driven) ports define what infrastructure must provide.
//!
//! These ports are implemented by adapters (database clients, HTTP clients, etc.)
//! and consumed by the application/domain layer.

pub mod repository;
pub mod cache;
pub mod queue;
pub mod event_bus;
pub mod config;
pub mod logger;
pub mod http;
pub mod filesystem;

pub use repository::*;
pub use cache::*;
pub use queue::*;
pub use event_bus::*;
pub use config::*;
pub use logger::*;
pub use http::*;
pub use filesystem::*;
