//! # Adapters Layer
//!
//! Infrastructure adapters implement outbound ports.
//!
//! ## Adapter Types
//!
//! ### Outbound Adapters (Secondary)
//! - `persistence/` - Database implementations
//!
//! ## Dependency Rule
//!
//! ```text
//! Outbound Adapter ──implements──► Domain Outbound Ports
//! ```

#[cfg(feature = "persistence-sqlx")]
pub mod persistence_sqlx;

#[cfg(feature = "persistence-sqlite")]
pub mod persistence_sqlite;
