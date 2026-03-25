//! # Agent Factory
//!
//! Factory for creating agent aggregates.

use crate::entities::Agent;
use crate::value_objects::{AgentId, AgentName, AgentStatus};

/// Factory for creating Agent entities.
pub struct AgentFactory;

impl AgentFactory {
    /// Creates a new agent with the given id and name.
    pub fn create(id: AgentId, name: AgentName) -> Result<Agent, crate::errors::DomainError> {
        Agent::new(id, name, AgentStatus::Active)
    }

    /// Creates a new agent with auto-generated ID.
    pub fn create_with_auto_id(name: AgentName) -> Result<Agent, crate::errors::DomainError> {
        let id = AgentId::new(format!("agent-{}", uuid_simple()))?;
        Self::create(id, name)
    }
}

/// Simple UUID v4-like generator (for zero-dep domain).
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", now)
}
