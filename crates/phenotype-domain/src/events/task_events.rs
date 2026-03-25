//! # Task Events
//!
//! Events emitted by task aggregates.

/// Domain event types for tasks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskEventType {
    Created,
    Assigned,
    Started,
    Completed,
    Failed,
    Cancelled,
}

/// Event: A new task was created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCreatedEvent {
    pub task_id: String,
    pub name: String,
    pub created_at: i64,
}

/// Event: Task was assigned to an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskAssignedEvent {
    pub task_id: String,
    pub agent_id: String,
    pub assigned_at: i64,
}

/// Event: Task execution started.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStartedEvent {
    pub task_id: String,
    pub started_at: i64,
}

/// Event: Task completed successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCompletedEvent {
    pub task_id: String,
    pub output: Option<String>,
    pub completed_at: i64,
}

/// Event: Task failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailedEvent {
    pub task_id: String,
    pub error: String,
    pub failed_at: i64,
}

/// Marker enum for all task events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskEvent {
    Created(TaskCreatedEvent),
    Assigned(TaskAssignedEvent),
    Started(TaskStartedEvent),
    Completed(TaskCompletedEvent),
    Failed(TaskFailedEvent),
}
