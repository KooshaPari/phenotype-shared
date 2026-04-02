//!
//! # Use Case Handlers
//!
//! Handlers implement the **application layer use cases**.
//! Each handler takes a command/query and coordinates domain + ports.
//!
//! ## Handler Responsibilities (Clean Architecture)
//!
//! 1. **Validation**: Validate input (command/query)
//! 2. **Authorization**: Check permissions (via inbound port)
//! 3. **Domain Logic**: Delegate to domain layer
//! 4. **Persistence**: Use outbound ports to save changes
//! 5. **Events**: Dispatch domain events to event bus
//! 6. **DTO**: Return appropriate response
//!
//! ## Transaction Boundaries
//!
//! Each handler runs in a single transaction. For multi-aggregate operations,
//! use the **Unit of Work** pattern via the repository port.

mod agent;
mod task;

pub use agent::{AgentCommandHandler, AgentQueryHandler};
pub use task::{TaskCommandHandler, TaskQueryHandler};

// Re-export commands and queries for tests
pub use crate::commands::*;
pub use crate::queries::*;

#[cfg(test)]
mod tests;
