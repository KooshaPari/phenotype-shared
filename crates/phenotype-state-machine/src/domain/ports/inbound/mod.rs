//! # Inbound Ports (Primary/Driving Ports)
//!
//! These ports define the **use cases** that the application exposes.
//! Inbound adapters (UI, API, CLI) call these ports to trigger business logic.
//!
//! ## Example
//!
//! ```rust,ignore
//! use crate::domain::ports::inbound::StateMachineCommandPort;
//!
//! // Application use case implements the port
//! impl StateMachineCommandPort for TransitionUseCase {
//!     fn execute_transition(&self, cmd: TransitionCommand) -> Result<TransitionRecord, Error> {
//!         // Business logic here
//!     }
//! }
//! ```

// Add inbound port traits here
// Example: use_cases/mod.rs would define traits like CreateOrderPort
