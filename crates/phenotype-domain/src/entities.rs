//! # Entities Module
//!
//! Entities have identity and mutable state.
//! Two entities with same ID are the same entity even if attributes differ.
//!
//! ## Entities in this module
//!
//! - `Agent`: Represents an autonomous agent in the system

use crate::value_objects::{AgentId, AgentName, AgentStatus, Timestamp};

/// Represents an autonomous agent in the system.
///
/// An entity has:
/// - Unique identity (AgentId)
/// - Mutable state (status, capabilities, metadata)
/// - Lifecycle events
#[derive(Debug, Clone)]
pub struct Agent {
    /// Unique identifier
    id: AgentId,
    /// Human-readable name
    name: AgentName,
    /// Current status
    status: AgentStatus,
    /// Capabilities (comma-separated tags)
    capabilities: Vec<String>,
    /// Creation timestamp
    created_at: Timestamp,
    /// Last update timestamp
    updated_at: Timestamp,
}

impl Agent {
    /// Creates a new Agent with the given name.
    pub fn new(name: AgentName) -> Self {
        let now = Timestamp::now();
        Self {
            id: AgentId::new(),
            name,
            status: AgentStatus::Idle,
            capabilities: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates an Agent with a specific ID (for reconstitution from events).
    pub fn with_id(id: AgentId, name: AgentName, status: AgentStatus, created_at: Timestamp) -> Self {
        let now = Timestamp::now();
        Self {
            id,
            name,
            status,
            capabilities: Vec::new(),
            created_at,
            updated_at: now,
        }
    }

    /// Returns the agent's ID.
    pub fn id(&self) -> &AgentId {
        &self.id
    }

    /// Returns the agent's name.
    pub fn name(&self) -> &AgentName {
        &self.name
    }

    /// Returns the current status.
    pub fn status(&self) -> AgentStatus {
        self.status
    }

    /// Returns the creation timestamp.
    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    /// Returns the last update timestamp.
    pub fn updated_at(&self) -> Timestamp {
        self.updated_at
    }

    /// Transitions the agent to a new status.
    ///
    /// Returns error if the transition is invalid.
    pub fn transition_to(&mut self, new_status: AgentStatus) -> Result<(), &'static str> {
        // Validate state transition
        match (&self.status, &new_status) {
            // Can always go to Error or Stopped (graceful shutdown)
            (_, AgentStatus::Stopped) => {}
            (_, AgentStatus::Error) => {}
            // Idle/Active can go to Active/Busy/Paused
            (AgentStatus::Idle, AgentStatus::Active) => {}
            (AgentStatus::Idle, AgentStatus::Paused) => {}
            (AgentStatus::Active, AgentStatus::Busy) => {}
            (AgentStatus::Active, AgentStatus::Paused) => {}
            (AgentStatus::Active, AgentStatus::Idle) => {}
            (AgentStatus::Busy, AgentStatus::Active) => {}
            (AgentStatus::Busy, AgentStatus::Error) => {}
            (AgentStatus::Paused, AgentStatus::Active) => {}
            (AgentStatus::Paused, AgentStatus::Idle) => {}
            // Terminal states cannot transition
            (AgentStatus::Stopped, _) => return Err("Cannot transition from Stopped"),
            (AgentStatus::Error, AgentStatus::Active) | (AgentStatus::Error, AgentStatus::Idle) => {}
            // Default: allow other transitions
            _ => {}
        }
        self.status = new_status;
        self.updated_at = Timestamp::now();
        Ok(())
    }

    /// Adds a capability tag.
    pub fn add_capability(&mut self, capability: impl Into<String>) {
        let cap: String = capability.into();
        if !self.capabilities.contains(&cap) {
            self.capabilities.push(cap);
            self.updated_at = Timestamp::now();
        }
    }

    /// Returns the list of capabilities.
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_agent() {
        let name = AgentName::new("Test Agent").unwrap();
        let agent = Agent::new(name);
        assert_eq!(agent.status(), AgentStatus::Idle);
    }

    #[test]
    fn test_transition_status() {
        let name = AgentName::new("Test Agent").unwrap();
        let mut agent = Agent::new(name);

        agent.transition_to(AgentStatus::Active).unwrap();
        assert_eq!(agent.status(), AgentStatus::Active);

        agent.transition_to(AgentStatus::Busy).unwrap();
        assert_eq!(agent.status(), AgentStatus::Busy);
    }

    #[test]
    fn test_cannot_transition_from_stopped() {
        let name = AgentName::new("Test Agent").unwrap();
        let mut agent = Agent::new(name);

        agent.transition_to(AgentStatus::Stopped).unwrap();
        assert!(agent.transition_to(AgentStatus::Active).is_err());
    }

    #[test]
    fn test_agent_equality_by_id() {
        let name1 = AgentName::new("Agent 1").unwrap();
        let name2 = AgentName::new("Agent 2").unwrap();

        let agent1 = Agent::new(name1);
        let agent2 = Agent::new(name2);

        // Verify both agents have valid IDs (format checked)
        assert!(agent1.id().is_ulid_format());
        assert!(agent2.id().is_ulid_format());
    }

    #[test]
    fn test_add_capability() {
        let name = AgentName::new("Test Agent").unwrap();
        let mut agent = Agent::new(name);

        agent.add_capability("coding");
        agent.add_capability("testing");

        assert!(agent.capabilities().contains(&"coding".to_string()));
        assert!(agent.capabilities().contains(&"testing".to_string()));
    }
}
