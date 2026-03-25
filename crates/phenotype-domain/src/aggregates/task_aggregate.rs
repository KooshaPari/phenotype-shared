//! # Task Aggregate
//!
//! Task aggregate root for task management.

use crate::errors::{DomainError, DomainResult};
use crate::value_objects::{TaskId, TaskName, TaskStatus, Priority, AgentId};

/// Task aggregate root.
#[derive(Debug, Clone)]
pub struct TaskAggregate {
    id: TaskId,
    name: TaskName,
    status: TaskStatus,
    priority: Priority,
    assigned_agent: Option<AgentId>,
    version: u64,
}

impl TaskAggregate {
    /// Creates a new task in Pending state.
    pub fn new(id: TaskId, name: TaskName, priority: Priority) -> DomainResult<Self> {
        if !matches!(priority, Priority::Low | Priority::Normal | Priority::High | Priority::Critical) {
            return Err(DomainError::Validation(crate::errors::ValidationError::new(
                "priority", "invalid priority",
            )));
        }
        Ok(Self {
            id,
            name,
            status: TaskStatus::Pending,
            priority,
            assigned_agent: None,
            version: 1,
        })
    }

    /// Assigns task to an agent.
    pub fn assign_to(&mut self, agent_id: AgentId) -> DomainResult<()> {
        if self.status != TaskStatus::Pending {
            return Err(DomainError::InvalidState(
                "Cannot assign non-pending task".into(),
            ));
        }
        self.assigned_agent = Some(agent_id);
        Ok(())
    }

    /// Starts task execution.
    pub fn start(&mut self) -> DomainResult<()> {
        if self.assigned_agent.is_none() {
            return Err(DomainError::InvalidState(
                "Cannot start unassigned task".into(),
            ));
        }
        if self.status != TaskStatus::Pending {
            return Err(DomainError::InvalidState(
                format!("Cannot start task in {:?} state", self.status),
            ));
        }
        self.status = TaskStatus::Running;
        self.version += 1;
        Ok(())
    }

    /// Completes task.
    pub fn complete(&mut self) -> DomainResult<()> {
        if self.status != TaskStatus::Running {
            return Err(DomainError::InvalidState(
                "Cannot complete non-running task".into(),
            ));
        }
        self.status = TaskStatus::Completed;
        self.version += 1;
        Ok(())
    }

    /// Fails task with error.
    pub fn fail(&mut self) -> DomainResult<()> {
        if self.status != TaskStatus::Running {
            return Err(DomainError::InvalidState(
                "Cannot fail non-running task".into(),
            ));
        }
        self.status = TaskStatus::Failed;
        self.version += 1;
        Ok(())
    }

    /// Cancels task.
    pub fn cancel(&mut self) -> DomainResult<()> {
        if self.status.is_terminal() {
            return Err(DomainError::InvalidState(
                "Cannot cancel terminal task".into(),
            ));
        }
        self.status = TaskStatus::Cancelled;
        self.version += 1;
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &TaskId { &self.id }
    pub fn name(&self) -> &TaskName { &self.name }
    pub fn status(&self) -> TaskStatus { self.status }
    pub fn priority(&self) -> Priority { self.priority }
    pub fn assigned_agent(&self) -> Option<&AgentId> { self.assigned_agent.as_ref() }
    pub fn version(&self) -> u64 { self.version }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_task() -> TaskAggregate {
        TaskAggregate::new(
            TaskId::new("task-001").unwrap(),
            TaskName::new("Test Task").unwrap(),
            Priority::Normal,
        ).unwrap()
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = make_task();
        assert_eq!(task.status(), TaskStatus::Pending);

        task.assign_to(AgentId::new("agent-001").unwrap()).unwrap();
        assert!(task.assigned_agent().is_some());

        task.start().unwrap();
        assert_eq!(task.status(), TaskStatus::Running);

        task.complete().unwrap();
        assert_eq!(task.status(), TaskStatus::Completed);
    }

    #[test]
    fn test_invalid_start_without_assignment() {
        let mut task = make_task();
        let result = task.start();
        assert!(result.is_err());
    }
}
