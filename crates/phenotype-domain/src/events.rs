//! # Domain Events
//!
//! Domain events are immutable facts that something happened.
//! They are the basis for event sourcing and CQRS.
//!
//! ## Principles
//!
//! - **Immutable**: Once created, cannot be modified
//! - **Descriptive**: Named in past tense (AgentCreated, not CreateAgent)
//! - **Complete**: Contains all data needed to reconstruct state
//! - **Time-ordered**: Includes timestamp for ordering

use crate::value_objects::{AgentId, AgentStatus, TaskId, TaskStatus, Timestamp};

/// Represents a domain event.
pub trait DomainEvent: Send + Sync {
    /// Returns the event type name.
    fn event_type(&self) -> &'static str;

    /// Returns the event timestamp.
    fn occurred_at(&self) -> Timestamp;

    /// Returns the aggregate ID this event belongs to.
    fn aggregate_id(&self) -> &str;
}

// ------------------------------------------------------------------------------------------------
// Agent Events
// ------------------------------------------------------------------------------------------------

/// Event fired when an agent is created.
#[derive(Debug, Clone)]
pub struct AgentCreated {
    pub agent_id: AgentId,
    pub name: String,
    pub occurred_at: Timestamp,
}

impl AgentCreated {
    pub fn new(agent_id: AgentId, name: String) -> Self {
        Self {
            agent_id,
            name,
            occurred_at: Timestamp::now(),
        }
    }
}

impl DomainEvent for AgentCreated {
    fn event_type(&self) -> &'static str {
        "AgentCreated"
    }

    fn occurred_at(&self) -> Timestamp {
        self.occurred_at
    }

    fn aggregate_id(&self) -> &str {
        self.agent_id.as_str()
    }
}

/// Event fired when an agent's status changes.
#[derive(Debug, Clone)]
pub struct AgentStatusChanged {
    pub agent_id: AgentId,
    pub old_status: AgentStatus,
    pub new_status: AgentStatus,
    pub occurred_at: Timestamp,
}

impl AgentStatusChanged {
    pub fn new(agent_id: AgentId, old_status: AgentStatus, new_status: AgentStatus) -> Self {
        Self {
            agent_id,
            old_status,
            new_status,
            occurred_at: Timestamp::now(),
        }
    }
}

impl DomainEvent for AgentStatusChanged {
    fn event_type(&self) -> &'static str {
        "AgentStatusChanged"
    }

    fn occurred_at(&self) -> Timestamp {
        self.occurred_at
    }

    fn aggregate_id(&self) -> &str {
        self.agent_id.as_str()
    }
}

// ------------------------------------------------------------------------------------------------
// Task Events
// ------------------------------------------------------------------------------------------------

/// Event fired when a task is created.
#[derive(Debug, Clone)]
pub struct TaskCreated {
    pub task_id: TaskId,
    pub name: String,
    pub occurred_at: Timestamp,
}

impl TaskCreated {
    pub fn new(task_id: TaskId, name: String) -> Self {
        Self {
            task_id,
            name,
            occurred_at: Timestamp::now(),
        }
    }
}

impl DomainEvent for TaskCreated {
    fn event_type(&self) -> &'static str {
        "TaskCreated"
    }

    fn occurred_at(&self) -> Timestamp {
        self.occurred_at
    }

    fn aggregate_id(&self) -> &str {
        self.task_id.as_str()
    }
}

/// Event fired when a task's status changes.
#[derive(Debug, Clone)]
pub struct TaskStatusChanged {
    pub task_id: TaskId,
    pub old_status: TaskStatus,
    pub new_status: TaskStatus,
    pub occurred_at: Timestamp,
}

impl TaskStatusChanged {
    pub fn new(task_id: TaskId, old_status: TaskStatus, new_status: TaskStatus) -> Self {
        Self {
            task_id,
            old_status,
            new_status,
            occurred_at: Timestamp::now(),
        }
    }
}

impl DomainEvent for TaskStatusChanged {
    fn event_type(&self) -> &'static str {
        "TaskStatusChanged"
    }

    fn occurred_at(&self) -> Timestamp {
        self.occurred_at
    }

    fn aggregate_id(&self) -> &str {
        self.task_id.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_created_event() {
        let agent_id = AgentId::new();
        let event = AgentCreated::new(agent_id.clone(), "Test Agent".into());

        assert_eq!(event.event_type(), "AgentCreated");
        assert_eq!(event.aggregate_id(), agent_id.as_str());
    }

    #[test]
    fn test_task_status_changed() {
        let task_id = TaskId::new();
        let event =
            TaskStatusChanged::new(task_id.clone(), TaskStatus::Pending, TaskStatus::Running);

        assert_eq!(event.event_type(), "TaskStatusChanged");
        assert_eq!(event.old_status, TaskStatus::Pending);
        assert_eq!(event.new_status, TaskStatus::Running);
    }
}
