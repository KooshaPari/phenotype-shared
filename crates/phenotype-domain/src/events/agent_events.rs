//! # Agent Events
//!
//! Events emitted by agent aggregates.

/// Domain event types for agents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentEventType {
    Created,
    Activated,
    Paused,
    Resumed,
    Stopping,
    Stopped,
    Error,
    Deleted,
}

/// Event: A new agent was created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCreatedEvent {
    pub agent_id: String,
    pub name: String,
    pub created_at: i64, // Unix millis
}

/// Event: Agent was activated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentActivatedEvent {
    pub agent_id: String,
    pub activated_at: i64,
}

/// Event: Agent was paused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentPausedEvent {
    pub agent_id: String,
    pub paused_at: i64,
}

/// Event: Agent was resumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentResumedEvent {
    pub agent_id: String,
    pub resumed_at: i64,
}

/// Event: Agent was stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStoppedEvent {
    pub agent_id: String,
    pub stopped_at: i64,
}

/// Event: Agent encountered an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentErrorEvent {
    pub agent_id: String,
    pub error: String,
    pub error_at: i64,
}

/// Marker enum for all agent events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentEvent {
    Created(AgentCreatedEvent),
    Activated(AgentActivatedEvent),
    Paused(AgentPausedEvent),
    Resumed(AgentResumedEvent),
    Stopped(AgentStoppedEvent),
    Error(AgentErrorEvent),
}
