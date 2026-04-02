//! # Outbound Ports
//!
//! Outbound (secondary/driven) ports define what infrastructure must provide.
//!
//! These ports are implemented by adapters (database clients, HTTP clients, etc.)
//! and consumed by the application/domain layer.

pub mod cache;
pub mod config;
pub mod event_bus;
pub mod filesystem;
pub mod http;
pub mod logger;
pub mod queue;
pub mod repository;

pub use cache::*;
pub use config::*;
pub use event_bus::*;
pub use filesystem::*;
pub use http::*;
pub use logger::*;
pub use queue::*;
pub use repository::*;
