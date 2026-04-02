//! # Command Ports
//!
//! Command ports define write operations (CQRS pattern).

use crate::error::Result;
use async_trait::async_trait;
use serde::Deserialize;

/// A command represents an intent to change state.
pub trait Command: Send + Sync + serde::Serialize + 'static {
    /// The type of the result after handling this command.
    type Result: for<'de> Deserialize<'de> + Send;
}

/// Command handler port.
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// Handle a command and return a result.
    async fn handle(&self, command: C) -> Result<C::Result>;
}

/// Command bus for dispatching commands to handlers.
#[async_trait]
pub trait CommandBus: Send + Sync {
    /// The command type.
    type Command: Command;

    /// Dispatch a command to its handler.
    async fn dispatch(&self, command: Self::Command) -> Result<<Self::Command as Command>::Result>;
}

/// Extension trait for command bus with convenience methods.
#[async_trait]
pub trait CommandBusExt: CommandBus {
    /// Dispatch with a specific handler type.
    async fn send<H>(&self, command: Self::Command) -> Result<<Self::Command as Command>::Result>
    where
        H: CommandHandler<Self::Command>;
}
