//! # Task Status
//!
//! Status state machine for task lifecycle.

/// Status of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskStatus {
    /// Task is pending and not yet started.
    #[default]
    Pending,
    /// Task is queued for execution.
    Queued,
    /// Task is currently running.
    Running,
    /// Task completed successfully.
    Completed,
    /// Task failed with an error.
    Failed,
    /// Task was cancelled.
    Cancelled,
    /// Task is paused.
    Paused,
}

impl TaskStatus {
    /// Returns true if the task is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }

    /// Returns true if the task can be cancelled.
    pub fn can_cancel(&self) -> bool {
        matches!(
            self,
            TaskStatus::Pending | TaskStatus::Queued | TaskStatus::Running | TaskStatus::Paused
        )
    }

    /// Returns true if the task can be retried.
    pub fn can_retry(&self) -> bool {
        matches!(self, TaskStatus::Failed)
    }

    /// Returns a short string code.
    pub fn code(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "PENDING",
            TaskStatus::Queued => "QUEUED",
            TaskStatus::Running => "RUNNING",
            TaskStatus::Completed => "COMPLETED",
            TaskStatus::Failed => "FAILED",
            TaskStatus::Cancelled => "CANCELLED",
            TaskStatus::Paused => "PAUSED",
        }
    }
}

impl core::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl core::str::FromStr for TaskStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(TaskStatus::Pending),
            "QUEUED" => Ok(TaskStatus::Queued),
            "RUNNING" => Ok(TaskStatus::Running),
            "COMPLETED" => Ok(TaskStatus::Completed),
            "FAILED" => Ok(TaskStatus::Failed),
            "CANCELLED" => Ok(TaskStatus::Cancelled),
            "PAUSED" => Ok(TaskStatus::Paused),
            _ => Err("unknown task status"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_terminal() {
        assert!(!TaskStatus::Pending.is_terminal());
        assert!(!TaskStatus::Running.is_terminal());
        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Cancelled.is_terminal());
    }

    #[test]
    fn test_can_cancel() {
        assert!(TaskStatus::Pending.can_cancel());
        assert!(TaskStatus::Queued.can_cancel());
        assert!(TaskStatus::Running.can_cancel());
        assert!(!TaskStatus::Completed.can_cancel());
        assert!(!TaskStatus::Failed.can_cancel());
    }

    #[test]
    fn test_can_retry() {
        assert!(TaskStatus::Failed.can_retry());
        assert!(!TaskStatus::Completed.can_retry());
    }
}
