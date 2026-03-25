//! # Domain Layer
//!
//! Pure domain logic for the state machine with **ZERO external dependencies**.
//!
//! This layer contains:
//! - [`entities`] - Core StateMachine, State trait, TransitionRecord
//! - [`services`] - Domain services for state machine operations
//! - [`ports`] - Port interfaces for persistence and external services
//!
//! ## Dependency Rule
//!
//! ```text
//! Domain ──NO DEPENDENCIES──► Application/Adapters
//! ```

pub mod entities;
pub mod services;
pub mod ports;

// Re-export commonly used types
pub use entities::*;
pub use services::*;
pub use ports::*;
